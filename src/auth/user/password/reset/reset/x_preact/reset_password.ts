import { h } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/node"

import { remoteCommonErrorReason } from "../../../../../../common/util/remote/x_error/reason"

import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { siteInfo } from "../../../../../../x_content/site"
import { appendScript } from "../../../../../sign/x_preact/script"
import { signNav } from "../../../../../sign/nav/x_preact/nav"
import {
    takeLongtimeField,
    ValidateBoardMessage,
} from "../../../../../../common/x_preact/design/form"

import { ApplicationError } from "../../../../../../avail/x_preact/application_error"
import { ClearChangesButton } from "../../../../../../common/x_preact/button/clear_changes_button"
import { ChangeButton } from "../../../../../../common/x_preact/button/change_button"
import { AuthUserPasswordField } from "../../../input/field/x_preact/input"

import { ResetPasswordAction } from "../action"
import { SignLink } from "../../../../../sign/nav/action"

import { ResetPasswordError } from "../data"

type Props = Readonly<{
    link: SignLink
    reset: ResetPasswordAction
}>
export function ResetPassword(props: Props): PreactNode {
    useLoadScript(props.reset)

    const resetState = useAtom(props.reset.state)

    switch (resetState.type) {
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
            return h(ApplicationError, { err: resetState.err.err })
    }

    return loginBox(siteInfo, {
        form: true,
        title: "パスワードリセット",
        body: [
            h(AuthUserPasswordField, {
                field: props.reset.newPassword,
                help: ["新しいパスワードを入力してください"],
                autocomplete: "new-password",
            }),
            buttons({ left: h(Submit, {}), right: h(Reset, {}) }),
            h(ValidateBoardMessage, { state: props.reset.validate }),
            h(Message, {}),
        ],
        footer: footerLinks(),
    })

    function Submit(_props: unknown): PreactNode {
        return h(ChangeButton, {
            label: "パスワードリセット",
            connect: props.reset.connect,
            validate: props.reset.validate,
            observe: props.reset.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.reset.submit()
        }
    }

    function Reset(_props: unknown): PreactNode {
        return h(ClearChangesButton, {
            observe: props.reset.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.reset.reset()
        }
    }

    function Message(_props: unknown): PreactNode {
        switch (resetState.type) {
            case "initial-reset":
            case "try-to-load":
            case "succeed-to-renew":
            case "ticket-not-expired":
            case "required-to-login":
            case "failed-to-renew":
            case "repository-error":
            case "load-error":
                return html``

            case "try-to-reset":
                if (resetState.hasTakenLongtime) {
                    return takeLongtimeField("パスワードリセット")
                }
                return html``

            case "failed-to-reset":
                return fieldHelp_error(resetError(resetState.err))
        }
    }

    function footerLinks() {
        return buttons({ left: privacyPolicyLink(), right: sendLink() })
    }
    function privacyPolicyLink() {
        return signNav(props.link.getNav_static_privacyPolicy())
    }
    function sendLink() {
        return signNav(props.link.getNav_password_reset_requestToken_retry())
    }
}

function resetError(err: ResetPasswordError): readonly PreactContent[] {
    switch (err.type) {
        case "empty-reset-token":
            return ["リセットトークンが指定されていません"]

        case "invalid-reset":
            return ["リセットトークンの有効期限が切れています"]

        case "already-reset":
            return [
                "すでにリセット済みです",
                "もう一度リセットする場合はトークンの送信からやり直してください",
            ]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりリセットに失敗しました`,
                ...reason.detail,
            ])
    }
}

function useLoadScript(reset: ResetPasswordAction): void {
    const resetState = useAtom(reset.state)

    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (resetState.type) {
            case "try-to-load":
                if (!resetState.scriptPath.valid) {
                    reset.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${resetState.type}`,
                    })
                    break
                }
                appendScript(resetState.scriptPath.value, (script) => {
                    script.onerror = () => {
                        reset.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${resetState.type}`,
                        })
                    }
                })
                break
        }
    }, [reset, resetState])
}
