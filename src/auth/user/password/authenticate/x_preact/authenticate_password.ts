import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../../z_vendor/getto-css/preact/layout/login"
import { buttons, fieldError, form } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"
import { siteInfo } from "../../../../../x_content/site"
import { lnir } from "../../../../../z_lib/ui/icon/init/line_icon"
import { icon_spinner } from "../../../../../x_content/icon"
import { appendScript } from "../../../../sign/x_preact/script"
import { signNav } from "../../../../sign/nav/x_preact/nav"
import { iconHtml } from "../../../../../core/x_preact/design/icon"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { InputLoginId } from "../../../login_id/input/x_preact/input"
import { InputPassword } from "../../input/x_preact/input"
import { SendButton } from "../../../../../core/x_preact/button/send_button"
import { ClearChangesButton } from "../../../../../core/x_preact/button/clear_changes_button"

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
    const observeState = useApplicationAction(props.authenticate.observe)

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

    const content = {
        title: "ログイン",
    }

    switch (state.type) {
        case "initial-login":
        case "failed-to-login":
        case "try-to-login":
            return authenticateForm(state)

        case "take-longtime-to-login":
            return loginBox(siteInfo, {
                ...content,
                body: [
                    html`<p>${iconHtml(icon_spinner)} 認証に時間がかかっています</p>`,
                    html`<p>
                        30秒以上かかる場合は何かがおかしいので、
                        <br />
                        お手数ですが管理者に連絡お願いします
                    </p>`,
                ],
                footer: footerLinks(),
            })

        case "try-to-load":
            // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
            return html``

        case "succeed-to-renew":
        case "ticket-not-expired":
        case "required-to-login":
        case "failed-to-renew":
            // これらはスクリプトがロードされた後に発行される
            // したがって、un-mount されているのでここには来ない
            return html``

        case "repository-error":
        case "load-error":
            return h(ApplicationError, { err: state.err.err })
    }

    type AuthenticateState =
        | Readonly<{ type: "initial-login" }>
        | Readonly<{ type: "try-to-login" }>
        | Readonly<{ type: "failed-to-login"; err: AuthenticatePasswordError }>
    function authenticateForm(authenticateState: AuthenticateState): VNode {
        return form(
            loginBox(siteInfo, {
                ...content,
                body: [
                    h(InputLoginId, {
                        field: props.authenticate.loginId,
                        autocomplete: "username",
                    }),
                    h(InputPassword, {
                        field: props.authenticate.password,
                        autocomplete: "current-password",
                    }),
                    buttons({
                        left: authenticateButton(),
                        right: clearButton(),
                    }),
                ],
                footer: [footerLinks(), message()],
            }),
        )

        function authenticateButton(): VNode {
            return h(SendButton, {
                label: "ログイン",
                icon: lnir(["enter"]),
                isConnecting: authenticateState.type === "try-to-login",
                validateState,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.authenticate.submit()
            }
        }
        function clearButton(): VNode {
            return h(ClearChangesButton, { observeState, onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.authenticate.clear()
            }
        }

        function message(): VNode {
            switch (authenticateState.type) {
                case "initial-login":
                    switch (validateState) {
                        case "initial":
                        case "valid":
                            return html``

                        case "invalid":
                            return fieldError(["正しく入力されていません"])
                    }
                    break

                case "try-to-login":
                    return html``

                case "failed-to-login":
                    return fieldError(loginError(authenticateState.err))
            }
        }
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
