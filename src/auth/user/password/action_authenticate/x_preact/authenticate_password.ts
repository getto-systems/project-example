import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_details/_ui/remote/reason"

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
import { appendScript } from "../../../../sign/x_preact/script"
import { signNav } from "../../../../sign/action_nav/x_preact/nav"

import { ApplicationErrorComponent } from "../../../../../avail/x_preact/application_error"
import { InputLoginIDEntry } from "../../../login_id/input/action_input/x_preact/input"
import { InputPasswordEntry } from "../../action_input/x_preact/input"

import {
    AuthenticatePasswordView,
    AuthenticatePasswordResource,
    AuthenticatePasswordResourceState,
} from "../resource"

import { AuthenticatePasswordError } from "../../authenticate/data"

export function AuthenticatePasswordEntry(view: AuthenticatePasswordView): VNode {
    const action = useApplicationView(view)
    return h(AuthenticatePasswordComponent, {
        authenticate: action,
        state: useApplicationAction(action),
        validate: useApplicationAction(action.validate),
    })
}

type Props = AuthenticatePasswordResource & AuthenticatePasswordResourceState
export function AuthenticatePasswordComponent(props: Props): VNode {
    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (props.state.type) {
            case "try-to-load":
                if (!props.state.scriptPath.valid) {
                    props.authenticate.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${props.state.type}`,
                    })
                    break
                }
                appendScript(props.state.scriptPath.value, (script) => {
                    script.onerror = () => {
                        props.authenticate.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${props.state.type}`,
                        })
                    }
                })
                break
        }
    }, [props.authenticate, props.state])

    return basedOn(props)

    function basedOn({ state, validate }: AuthenticatePasswordResourceState): VNode {
        switch (state.type) {
            case "initial-login":
                return authenticateForm({ state: validate })

            case "failed-to-login":
                return authenticateForm({ state: validate, err: loginError(state.err) })

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
                return h(ApplicationErrorComponent, { err: state.err.err })
        }
    }

    type State = "initial" | "valid" | "invalid" | "connecting"

    type Content = Readonly<{ state: State }> | Readonly<{ state: State; err: VNodeContent[] }>

    function authenticateTitle() {
        return "ログイン"
    }

    function authenticateForm(content: Content): VNode {
        return form(
            loginBox(siteInfo, {
                title: authenticateTitle(),
                body: [
                    h(InputLoginIDEntry, { field: props.authenticate.loginID }),
                    h(InputPasswordEntry, { field: props.authenticate.password }),
                    buttons({ left: button(), right: clearButton() }),
                ],
                footer: [footerLinks(), error()],
            }),
        )

        function clearButton(): VNode {
            const label = "入力内容をクリア"
            switch (content.state) {
                case "initial":
                case "connecting":
                    return button_disabled({ label })

                case "invalid":
                case "valid":
                    return button_undo({ label, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.authenticate.clear()
            }
        }

        function button() {
            const label = "ログイン"

            switch (content.state) {
                case "initial":
                    return button_send({ state: "normal", label, onClick })

                case "valid":
                    return button_send({ state: "confirm", label, onClick })

                case "invalid":
                    return button_disabled({ label })

                case "connecting":
                    return button_send({
                        state: "connect",
                        label: html`ログインしています ${spinner}`,
                    })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.authenticate.submit()
            }
        }

        function error(): VNode {
            if ("err" in content) {
                return fieldError(content.err)
            }

            switch (content.state) {
                case "initial":
                case "valid":
                case "connecting":
                    return EMPTY_CONTENT

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
        return signNav(props.authenticate.link.getNav_static_privacyPolicy())
    }
    function resetLink() {
        return signNav(props.authenticate.link.getNav_password_reset_requestToken())
    }
}

function loginError(err: AuthenticatePasswordError): VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "invalid-password":
            return ["ログインIDかパスワードが違います"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により認証に失敗しました`,
                ...reason.detail,
            ])
    }
}

const EMPTY_CONTENT = html``
