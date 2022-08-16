import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { remoteCommonErrorReason } from "../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { loginBox } from "../../../../../z_vendor/getto-css/preact/layout/login"
import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"

import { siteInfo } from "../../../../../x_content/site"
import { lnir } from "../../../../../z_lib/ui/icon/init/line_icon"
import { appendScript } from "../../../../sign/x_preact/script"
import { signNav } from "../../../../sign/nav/x_preact/nav"
import { takeLongtimeField, ValidationMessage } from "../../../../../common/x_preact/design/form"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { LoginIdField } from "../../../login_id/input/x_preact/field"
import { PasswordField } from "../../input/x_preact/input"
import { SendButton } from "../../../../../common/x_preact/button/send_button"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"

import { AuthenticatePasswordAction } from "../action"
import { SignLink } from "../../../../sign/nav/action"

import { AuthenticatePasswordError } from "../data"

type Props = Readonly<{
    link: SignLink
    authenticate: AuthenticatePasswordAction
}>
export function AuthenticatePassword(props: Props): VNode {
    useLoadScript(props.authenticate)

    const authenticateState = useApplicationState(props.authenticate.state)
    switch (authenticateState.type) {
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
            return h(ApplicationError, { err: authenticateState.err.err })
    }

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
            buttons({ left: h(Submit, {}), right: h(Clear, {}) }),
            h(ValidationMessage, props.authenticate.validate),
            h(Message, {}),
        ],
        footer: footerLinks(),
    })

    function Submit(_props: unknown): VNode {
        const validateState = useApplicationState(props.authenticate.validate.state)
        const observeState = useApplicationState(props.authenticate.observe.state)

        return h(SendButton, {
            label: "ログイン",
            icon: lnir(["enter"]),
            isConnecting: authenticateState.type === "try-to-login",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.authenticate.submit()
        }
    }
    function Clear(_props: unknown): VNode {
        const observeState = useApplicationState(props.authenticate.observe.state)

        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.authenticate.clear()
        }
    }

    function Message(_props: unknown): VNode {
        switch (authenticateState.type) {
            case "initial-login":
            case "try-to-load":
            case "succeed-to-renew":
            case "ticket-not-expired":
            case "required-to-login":
            case "failed-to-renew":
            case "repository-error":
            case "load-error":
                return html``

            case "try-to-login":
                if (authenticateState.hasTakenLongtime) {
                    return takeLongtimeField("認証")
                }
                return html``

            case "failed-to-login":
                return fieldHelp_error(loginError(authenticateState.err))
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

function useLoadScript(authenticate: AuthenticatePasswordAction): void {
    const authenticateState = useApplicationState(authenticate.state)
    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (authenticateState.type) {
            case "try-to-load":
                if (!authenticateState.scriptPath.valid) {
                    authenticate.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${authenticateState.type}`,
                    })
                    break
                }
                appendScript(authenticateState.scriptPath.value, (script) => {
                    script.onerror = () => {
                        authenticate.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${authenticateState.type}`,
                        })
                    }
                })
                break
        }
    }, [authenticate, authenticateState])
}
