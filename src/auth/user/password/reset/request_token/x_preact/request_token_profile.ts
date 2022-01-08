import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/reason"

import { useApplicationAction } from "../../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../../ui/vendor/getto-css/preact/design/form"
import { spinner } from "../../../../../../example/x_preact/design/icon"
import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { InputLoginIDEntry } from "../../../../login_id/input/action_input/x_preact/input"

import { RequestResetTokenError } from "../data"
import { box } from "../../../../../../../ui/vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../../../ui/vendor/getto-css/preact/design/highlight"
import { RequestResetTokenProfileAction, RequestResetTokenProfileState } from "../action"
import { ValidateBoardActionState } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

type EntryProps = Readonly<{
    requestToken: RequestResetTokenProfileAction
}>
export function RequestResetTokenProfileEntry({ requestToken }: EntryProps): VNode {
    return h(RequestResetTokenProfileComponent, {
        requestToken,
        state: useApplicationAction(requestToken),
        validate: useApplicationAction(requestToken.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: RequestResetTokenProfileState
        validate: ValidateBoardActionState
    }>
export function RequestResetTokenProfileComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, validate }: Props): VNode {
        switch (state.type) {
            case "initial-request-token":
                return buttonBox({ type: "initial" })

            case "input-login-id":
                return formBox({ type: validate })

            case "try-to-request-token":
                return formBox({ type: "connecting" })

            case "take-longtime-to-request-token":
                return formBox({ type: "take-longtime" })

            case "succeed-to-request-token":
                return buttonBox({ type: "success" })

            case "failed-to-request-token":
                return formBox({ type: validate, err: requestTokenError(state.err) })
        }
    }

    type ButtonContentType = "initial" | "success"
    type ButtonContent = Readonly<{ type: ButtonContentType }>
    function buttonBox(state: ButtonContent): VNode {
        return form(box(content()))

        type BoxContent =
            | Readonly<{ title: VNodeContent; body: VNodeContent }>
            | Readonly<{ title: VNodeContent; body: VNodeContent; footer: VNodeContent }>
        function content(): BoxContent {
            switch (state.type) {
                case "initial":
                    return {
                        title: title(),
                        body: openButton(),
                    }

                case "success":
                    return {
                        title: title(),
                        body: openButton(),
                        footer: [
                            notice_success([
                                html`パスワードリセットのため、<br />
                                    トークンをメールで送信しました`,
                            ]),
                            html`<p>
                                メールからパスワードリセットできます<br />
                                メールを確認してください
                            </p>`,
                        ],
                    }
            }
        }
        function title() {
            return "パスワードリセット"
        }
        function openButton(): VNode {
            return button_send({ state: "normal", label: "トークン送信", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.requestToken.open()
            }
        }
    }

    type FormContentType = "initial" | "valid" | "invalid" | "connecting" | "take-longtime"
    type FormContent =
        | Readonly<{ type: FormContentType }>
        | Readonly<{ type: FormContentType; err: VNodeContent[] }>
    function formBox(state: FormContent): VNode {
        return form(
            box({
                title: "パスワードリセットトークン送信",
                body: [
                    h(InputLoginIDEntry, {
                        field: props.requestToken.loginID,
                        title: "ログインID",
                        help: ["確認のため、ログインIDを入力します"],
                    }),
                ],
                footer: [
                    buttons({
                        left: submitButton(),
                        right: clearButton(),
                    }),
                    ...message(),
                    buttons({
                        right: closeButton(),
                    }),
                ],
            }),
        )

        function submitButton(): VNode {
            const label = "トークン送信"

            switch (state.type) {
                case "initial":
                    return button_send({ state: "normal", label, onClick })

                case "valid":
                    return button_send({ state: "confirm", label, onClick })

                case "invalid":
                    return button_disabled({ label })

                case "connecting":
                case "take-longtime":
                    return button_send({
                        state: "connect",
                        label: html`リセットトークン送信中 ${spinner}`,
                    })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.requestToken.submit()
            }
        }

        function clearButton(): VNode {
            const label = "入力内容をクリア"
            switch (state.type) {
                case "initial":
                    return button_disabled({ label })

                case "connecting":
                case "take-longtime":
                    return EMPTY_CONTENT

                case "invalid":
                case "valid":
                    return button_undo({ label, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.requestToken.clear()
            }
        }
        function closeButton(): VNode {
            return button_undo({ label: "閉じる", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.requestToken.close()
            }
        }

        function message(): VNode[] {
            if ("err" in state) {
                return [fieldError(state.err)]
            }

            switch (state.type) {
                case "initial":
                case "valid":
                case "connecting":
                    return []

                case "take-longtime":
                    return [
                        fieldError([
                            html`${spinner} リセットトークン送信中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "invalid":
                    return [fieldError(["正しく入力されていません"])]
            }
        }
    }
}

function requestTokenError(err: RequestResetTokenError): VNodeContent[] {
    switch (err.type) {
        case "validation-error":
            return ["正しく入力してください"]

        case "invalid-reset":
            return ["ログインIDが登録されていないか、トークンの送信先が登録されていません"]

        default:
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}によりトークンの送信に失敗しました`,
                ...reason.detail,
            ])
    }
}

const EMPTY_CONTENT = html``
