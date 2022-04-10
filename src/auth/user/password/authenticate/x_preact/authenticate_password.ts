import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../../z_vendor/getto-css/preact/layout/login"
import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"
import { siteInfo } from "../../../../../x_content/site"
import { icon_spinner } from "../../../../../x_content/icon"
import { appendScript } from "../../../../sign/x_preact/script"
import { signNav } from "../../../../sign/nav/x_preact/nav"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { InputLoginId } from "../../../login_id/input/x_preact/input"
import { InputPassword } from "../../input/x_preact/input"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { AuthenticatePasswordAction } from "../action"
import { SignLink } from "../../../../sign/nav/action"

import { AuthenticatePasswordError } from "../data"

type Props = Readonly<{
    link: SignLink
    authenticate: ApplicationView<AuthenticatePasswordAction>
}>
export function AuthenticatePassword(viewProps: Props): VNode {
    const props = {
        link: viewProps.link,
        authenticate: useApplicationView(viewProps.authenticate),
    }
    const state = useApplicationAction(props.authenticate)
    const validateState = useApplicationAction(props.authenticate.validate)

    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (state.type) {
            case "try-to-load":
                if (!state.scriptPath.valid) {
                    props.authenticate.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${state.type}`,
                    })
                    break
                }
                appendScript(state.scriptPath.value, (script) => {
                    script.onerror = () => {
                        props.authenticate.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${state.type}`,
                        })
                    }
                })
                break
        }
    }, [props.authenticate, state])

    switch (state.type) {
        case "initial-login":
            return authenticateForm({ state: validateState })

        case "failed-to-login":
            return authenticateForm({ state: validateState, err: loginError(state.err) })

        case "try-to-login":
            return authenticateForm({ state: "connecting" })

        case "take-longtime-to-login":
            return takeLongtimeMessage()

        case "try-to-load":
            // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
            return EMPTY_CONTENT

        case "succeed-to-renew":
        case "ticket-not-expired":
        case "required-to-login":
        case "failed-to-renew":
            // これらはスクリプトがロードされた後に発行される
            // したがって、un-mount されているのでここには来ない
            return EMPTY_CONTENT

        case "repository-error":
        case "load-error":
            return h(ApplicationError, { err: state.err.err })
    }

    type State = "initial" | "valid" | "invalid" | "connecting"

    type Content = Readonly<{ state: State }> & Partial<{ err: readonly VNodeContent[] }>

    function authenticateTitle() {
        return "ログイン"
    }

    function authenticateForm(content: Content): VNode {
        return form(
            loginBox(siteInfo, {
                title: authenticateTitle(),
                body: [
                    h(InputLoginId, {
                        field: props.authenticate.loginId,
                        autocomplete: "username",
                    }),
                    h(InputPassword, {
                        field: props.authenticate.password,
                        autocomplete: "current-password",
                    }),
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
                        label: html`ログインしています ${icon_spinner}`,
                    })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.authenticate.submit()
            }
        }

        function error(): VNode {
            if (content.err) {
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
                html`<p>${icon_spinner} 認証中です</p>`,
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

function loginError(err: AuthenticatePasswordError): readonly VNodeContent[] {
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
