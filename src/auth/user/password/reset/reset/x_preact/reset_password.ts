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
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { loginBox } from "../../../../../../z_vendor/getto-css/preact/layout/login"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"
import { siteInfo } from "../../../../../../x_content/site"
import { icon_spinner } from "../../../../../../x_content/icon"
import { appendScript } from "../../../../../sign/x_preact/script"
import { signNav } from "../../../../../sign/nav/x_preact/nav"

import { ApplicationError } from "../../../../../../avail/x_preact/application_error"
import { InputLoginId } from "../../../../login_id/input/x_preact/input"
import { InputPassword } from "../../../input/x_preact/input"

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

    const content = {
        title: "パスワードリセット",
    }

    switch (state.type) {
        case "initial-reset":
        case "failed-to-reset":
        case "try-to-reset":
            return form(
                loginBox(siteInfo, {
                    ...content,
                    body: [
                        h(InputLoginId, {
                            field: props.reset.loginId,
                            help: ["入力したログインIDをもう一度入力してください"],
                            autocomplete: "username",
                        }),
                        h(InputPassword, {
                            field: props.reset.password,
                            help: ["新しいパスワードを入力してください"],
                            autocomplete: "new-password",
                        }),
                        buttons({
                            left:
                                state.type === "try-to-reset" ? connectingButton() : resetButton(),
                            right: clearButton(),
                        }),
                    ],
                    footer: [
                        footerLinks(),
                        state.type === "failed-to-reset" ? fieldError(resetError(state.err)) : "",
                    ],
                }),
            )

        case "take-longtime-to-reset":
            return loginBox(siteInfo, {
                ...content,
                body: [
                    html`<p>${icon_spinner} リセットに時間がかかっています</p>`,
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

    function clearButton() {
        const label = "入力内容をクリア"
        switch (validateState) {
            case "initial":
                return button_disabled({ label })

            case "invalid":
            case "valid":
                return button_undo({ label, onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.reset.clear()
        }
    }

    function resetButton() {
        const label = "パスワードリセット"

        switch (validateState) {
            case "initial":
                return button_send({ state: "normal", label, onClick })

            case "valid":
                return button_send({ state: "confirm", label, onClick })

            case "invalid":
                return button_disabled({ label })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.reset.submit()
        }
    }
    function connectingButton(): VNode {
        return button_send({
            state: "connect",
            label: html`パスワードをリセットしています ${icon_spinner}`,
        })
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
        case "validation-error":
            return ["正しく入力してください"]

        case "empty-reset-token":
            return ["リセットトークンが見つかりませんでした"]

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

const EMPTY_CONTENT = html``
