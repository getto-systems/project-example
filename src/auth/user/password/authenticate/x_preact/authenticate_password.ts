import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../../z_vendor/getto-css/preact/layout/login"
import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"

import { siteInfo } from "../../../../../x_content/site"
import { lnir } from "../../../../../z_lib/ui/icon/init/line_icon"
import { appendScript } from "../../../../sign/x_preact/script"
import { signNav } from "../../../../sign/nav/x_preact/nav"
import { takeLongtimeField } from "../../../../../core/x_preact/design/form"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { LoginIdField } from "../../../login_id/input/x_preact/input"
import { PasswordField } from "../../input/x_preact/input"
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

    switch (state.type) {
        case "initial-login":
        case "failed-to-login":
        case "try-to-login":
            return authenticateForm(state)

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
        | Readonly<{ type: "try-to-login"; hasTakenLongtime: boolean }>
        | Readonly<{ type: "failed-to-login"; err: AuthenticatePasswordError }>
    function authenticateForm(state: AuthenticateState): VNode {
        return loginBox(siteInfo, {
            form: true,
            title: "ログイン",
            body: [
                h(LoginIdField, {
                    field: props.authenticate.loginId,
                    autocomplete: "username",
                }),
                h(PasswordField, {
                    field: props.authenticate.password,
                    autocomplete: "current-password",
                }),
                buttons({
                    left: authenticateButton(),
                    right: clearButton(),
                }),
            ],
            footer: [footerLinks(), ...validationMessage(), ...message()],
        })

        function authenticateButton(): VNode {
            return h(SendButton, {
                label: "ログイン",
                icon: lnir(["enter"]),
                isConnecting: state.type === "try-to-login",
                validateState,
                observeState,
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

        function validationMessage(): VNode[] {
            switch (validateState) {
                case "initial":
                case "valid":
                    return []

                case "invalid":
                    return [fieldHelp_error(["正しく入力されていません"])]
            }
        }
        function message(): VNode[] {
            switch (state.type) {
                case "initial-login":
                    return []

                case "try-to-login":
                    if (state.hasTakenLongtime) {
                        return [takeLongtimeField("認証")]
                    }
                    return []

                case "failed-to-login":
                    return [fieldHelp_error(loginError(state.err))]
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
        case "invalid-password":
            return ["ログインIDかパスワードが違います"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により認証に失敗しました`,
                ...reason.detail,
            ])
    }
}
