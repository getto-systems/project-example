import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
    form,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"
import { siteInfo } from "../../../../../../x_content/site"
import { appendScript } from "../../../../../sign/x_preact/script"
import { signNav } from "../../../../../sign/nav/x_preact/nav"
import { takeLongtimeField, validationMessage } from "../../../../../../core/x_preact/design/form"

import { ApplicationError } from "../../../../../../avail/x_preact/application_error"
import { LoginIdField } from "../../../../login_id/input/x_preact/input"
import { PasswordField } from "../../../input/x_preact/input"
import { ClearChangesButton } from "../../../../../../core/x_preact/button/clear_changes_button"
import { ChangeButton } from "../../../../../../core/x_preact/button/change_button"

import { ApplicationView } from "../../../../../../z_vendor/getto-application/action/action"
import { ResetPasswordAction } from "../action"
import { SignLink } from "../../../../../sign/nav/action"

import { ResetPasswordError } from "../data"

type Props = Readonly<{
    link: SignLink
    reset: ApplicationView<ResetPasswordAction>
}>
export function ResetPassword(viewProps: Props): VNode {
    const props = {
        link: viewProps.link,
        reset: useApplicationView(viewProps.reset),
    }

    const state = useApplicationAction(props.reset)
    const validateState = useApplicationAction(props.reset.validate)
    const observeState = useApplicationAction(props.reset.observe)

    useLayoutEffect(() => {
        // スクリプトのロードは appendChild する必要があるため useLayoutEffect で行う
        switch (state.type) {
            case "try-to-load":
                if (!state.scriptPath.valid) {
                    props.reset.loadError({
                        type: "infra-error",
                        err: `スクリプトのロードに失敗しました: ${state.type}`,
                    })
                    break
                }
                appendScript(state.scriptPath.value, (script) => {
                    script.onerror = () => {
                        props.reset.loadError({
                            type: "infra-error",
                            err: `スクリプトのロードに失敗しました: ${state.type}`,
                        })
                    }
                })
                break
        }
    }, [props.reset, state])

    switch (state.type) {
        case "initial-reset":
        case "failed-to-reset":
        case "try-to-reset":
            return resetForm(state)

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

    type ResetState =
        | Readonly<{ type: "initial-reset" }>
        | Readonly<{ type: "try-to-reset"; hasTakenLongtime: boolean }>
        | Readonly<{ type: "failed-to-reset"; err: ResetPasswordError }>
    function resetForm(state: ResetState): VNode {
        return form(
            loginBox(siteInfo, {
                title: "パスワードリセット",
                body: [
                    h(LoginIdField, {
                        field: props.reset.loginId,
                        help: ["入力したログインIDをもう一度入力してください"],
                        autocomplete: "username",
                    }),
                    h(PasswordField, {
                        field: props.reset.password,
                        help: ["新しいパスワードを入力してください"],
                        autocomplete: "new-password",
                    }),
                    buttons({
                        left: resetButton(),
                        right: clearButton(),
                    }),
                ],
                footer: [footerLinks(), ...validationMessage(validateState), ...message()],
            }),
        )

        function resetButton() {
            return h(ChangeButton, {
                label: "パスワードリセット",
                isConnecting: state.type === "try-to-reset",
                validateState,
                observeState,
                onClick,
            })

            function onClick(e: Event) {
                e.preventDefault()
                props.reset.submit()
            }
        }

        function message(): readonly VNode[] {
            switch (state.type) {
                case "initial-reset":
                    return []

                case "try-to-reset":
                    if (state.hasTakenLongtime) {
                        return [takeLongtimeField("パスワードリセット")]
                    }
                    return []

                case "failed-to-reset":
                    return [fieldHelp_error(resetError(state.err))]
            }
        }
    }

    function clearButton() {
        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.reset.clear()
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

function resetError(err: ResetPasswordError): readonly VNodeContent[] {
    switch (err.type) {
        case "empty-reset-token":
            return ["リセットトークンが指定されていません"]

        case "invalid-reset":
            return ["ログインIDが最初に入力したものと違うか、有効期限が切れています"]

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
