import { h } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../common/x_preact/vnode"

import { remoteCommonErrorReason } from "../../../../../common/util/remote/x_error/reason"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { loginBox } from "../../../../../z_vendor/getto-css/preact/layout/login"
import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"

import { siteInfo } from "../../../../../x_content/site"
import { lnir } from "../../../../../common/util/icon/detail/line_icon"
import { appendScript } from "../../../../sign/x_preact/script"
import { signNav } from "../../../../sign/nav/x_preact/nav"
import { takeLongtimeField, ValidateBoardMessage } from "../../../../../common/x_preact/design/form"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { AuthUserLoginIdField } from "../../../login_id/input/field/x_preact/input"
import { AuthUserPasswordField } from "../../input/field/x_preact/input"
import { SendButton } from "../../../../../common/x_preact/button/send_button"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"

import { AuthenticatePasswordAction } from "../action"
import { SignLink } from "../../../../sign/nav/action"

import { AuthenticatePasswordError } from "../data"

type Props = Readonly<{
    link: SignLink
    authenticate: AuthenticatePasswordAction
}>
export function AuthenticatePassword(props: Props): PreactNode {
    useLoadScript(props.authenticate)

    const authenticateState = useAtom(props.authenticate.state)
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
            h(AuthUserLoginIdField, {
                field: props.authenticate.loginId,
                autocomplete: "username",
            }),
            h(AuthUserPasswordField, {
                field: props.authenticate.password,
                autocomplete: "current-password",
            }),
            buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
            h(ValidateBoardMessage, { state: props.authenticate.validate }),
            h(Message, {}),
        ],
        footer: footerLinks(),
    })

    function Submit(_props: unknown): PreactNode {
        return h(SendButton, {
            label: "ログイン",
            icon: lnir(["enter"]),
            connect: props.authenticate.connect,
            validate: props.authenticate.validate,
            observe: props.authenticate.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.authenticate.submit()
        }
    }
    function Reset(_props: unknown): PreactNode {
        return h(ClearChangesButton, {
            observe: props.authenticate.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.authenticate.reset()
        }
    }

    function Message(_props: unknown): PreactNode {
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

function loginError(err: AuthenticatePasswordError): readonly PreactContent[] {
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
    const authenticateState = useAtom(authenticate.state)
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
