import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../../../ui/vendor/getto-css/preact/layout/login"
import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../example/_ui/x_preact/design/common"
import { siteInfo } from "../../../../../example/site"
import { spinner } from "../../../../../example/_ui/x_preact/design/icon"
import { appendScript } from "../../../../_ui/common/x_preact/script"
import { signNav } from "../../../../_ui/common/nav/x_preact/nav"

import { ApplicationErrorComponent } from "../../../../../avail/_ui/x_preact/application_error"
import { InputLoginIDEntry } from "../../../../login_id/_ui/action_input/x_preact/input"
import { InputPasswordEntry } from "../../action_input/x_preact/input"

import {
    AuthenticatePasswordView,
    AuthenticatePasswordResource,
    AuthenticatePasswordResourceState,
} from "../resource"

import { AuthenticatePasswordError } from "../../authenticate/data"
import { remoteCommonError } from "../../../../../z_details/_ui/remote/helper"

export function AuthenticatePasswordEntry(view: AuthenticatePasswordView): VNode {
    const resource = useApplicationView(view)
    return h(AuthenticatePasswordComponent, {
        ...resource,
        state: {
            core: useApplicationAction(resource.authenticate.core),
            form: useApplicationAction(resource.authenticate.form.validate),
        },
    })
}

type Props = AuthenticatePasswordResource & AuthenticatePasswordResourceState
export function AuthenticatePasswordComponent(props: Props): VNode {
    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (props.state.core.type) {
            case "try-to-load":
                if (!props.state.core.scriptPath.valid) {
                    props.authenticate.core.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${props.state.core.type}`,
                    })
                    break
                }
                appendScript(props.state.core.scriptPath.value, (script) => {
                    script.onerror = () => {
                        props.authenticate.core.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${props.state.core.type}`,
                        })
                    }
                })
                break
        }
    }, [props.authenticate.core, props.state.core])

    switch (props.state.core.type) {
        case "initial-login":
            return authenticateForm({ state: "login" })

        case "failed-to-login":
            return authenticateForm({ state: "login", error: loginError(props.state.core.err) })

        case "try-to-login":
            return authenticateForm({ state: "connecting" })

        case "take-longtime-to-login":
            return takeLongtimeMessage()

        case "try-to-load":
            // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
            return EMPTY_CONTENT

        case "succeed-to-renew":
        case "authn-not-expired":
        case "required-to-login":
        case "failed-to-renew":
            // これらはスクリプトがロードされた後に発行される
            // したがって、un-mount されているのでここには来ない
            return EMPTY_CONTENT

        case "repository-error":
        case "load-error":
            return h(ApplicationErrorComponent, { err: props.state.core.err.err })
    }

    type AuthenticateFormState = "login" | "connecting"

    type AuthenticateFormContent =
        | AuthenticateFormContent_base
        | (AuthenticateFormContent_base & AuthenticateFormContent_error)

    type AuthenticateFormContent_base = Readonly<{ state: AuthenticateFormState }>
    type AuthenticateFormContent_error = Readonly<{ error: VNodeContent[] }>

    function authenticateTitle() {
        return "ログイン"
    }

    function authenticateForm(content: AuthenticateFormContent): VNode {
        return form(
            loginBox(siteInfo, {
                title: authenticateTitle(),
                body: [
                    h(InputLoginIDEntry, { field: props.authenticate.form.loginID, help: [] }),
                    h(InputPasswordEntry, { field: props.authenticate.form.password, help: [] }),
                    buttons({ left: button(), right: clearButton() }),
                ],
                footer: [footerLinks(), error()],
            }),
        )

        function clearButton() {
            const label = "入力内容をクリア"
            switch (props.state.form) {
                case "initial":
                    return button_disabled({ label })

                case "invalid":
                case "valid":
                    return button_undo({ label, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.authenticate.form.clear()
            }
        }

        function button() {
            switch (content.state) {
                case "login":
                    return loginButton()

                case "connecting":
                    return connectingButton()
            }

            function loginButton() {
                const label = "ログイン"

                switch (props.state.form) {
                    case "initial":
                        return button_send({ state: "normal", label, onClick })

                    case "valid":
                        return button_send({ state: "confirm", label, onClick })

                    case "invalid":
                        return button_disabled({ label })
                }

                function onClick(e: Event) {
                    e.preventDefault()
                    props.authenticate.core.submit(props.authenticate.form.validate.get())
                }
            }
            function connectingButton(): VNode {
                return button_send({
                    state: "connect",
                    label: html`ログインしています ${spinner}`,
                })
            }
        }

        function error() {
            if ("error" in content) {
                return fieldError(content.error)
            }

            switch (props.state.form) {
                case "initial":
                case "valid":
                    return ""

                case "invalid":
                    return fieldError(["正しく入力されていません"])
            }
        }
    }
    function takeLongtimeMessage() {
        return loginBox(siteInfo, {
            title: authenticateTitle(),
            body: [
                html`<p>${spinner} 認証中です</p>`,
                html`<p>
                    30秒以上かかる場合は何かがおかしいので、
                    <br />
                    お手数ですが管理者に連絡お願いします
                </p>`,
            ],
            footer: footerLinks(),
        })
    }

    function footerLinks() {
        return buttons({ left: privacyPolicyLink(), right: resetLink() })
    }
    function privacyPolicyLink() {
        return signNav(props.link.getNav_static_privacyPolicy())
    }
    function resetLink() {
        return signNav(props.link.getNav_password_reset_requestToken())
    }
}

function loginError(err: AuthenticatePasswordError): VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "invalid-password":
            return ["ログインIDかパスワードが違います"]

        default:
            return remoteCommonError(err, (reason) => [
                `${reason.message}により認証に失敗しました`,
                ...reason.detail,
            ])
    }
}

const EMPTY_CONTENT = html``
