import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../../../z_lib/ui/remote/x_error/reason"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { icon_spinner } from "../../../../../../x_content/icon"
import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { InputLoginIdEntry } from "../../../../login_id/input/x_preact/input"

import { RequestResetTokenError } from "../data"
import { box } from "../../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../../z_vendor/getto-css/preact/design/highlight"
import { RequestResetTokenAction, RequestResetTokenState } from "../action"
import { ValidateBoardState } from "../../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    EditableBoardAction,
    EditableBoardState,
} from "../../../../../../z_vendor/getto-application/board/editable/action"

type EntryProps = Readonly<{
    editable: EditableBoardAction
    requestToken: RequestResetTokenAction
}>
export function RequestResetTokenProfileEntry({ editable, requestToken }: EntryProps): VNode {
    return h(RequestResetTokenProfileComponent, {
        editable,
        requestToken,
        state: useApplicationAction(requestToken),
        editableState: useApplicationAction(editable),
        validateState: useApplicationAction(requestToken.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: RequestResetTokenState
        editableState: EditableBoardState
        validateState: ValidateBoardState
    }>
export function RequestResetTokenProfileComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, editableState, validateState }: Props): VNode {
        if (editableState.isEditable) {
            switch (state.type) {
                case "initial":
                case "success":
                    return formBox({ type: validateState })

                case "try":
                    return formBox({ type: "connecting" })

                case "take-longtime":
                    return formBox({ type: "take-longtime" })

                case "failed":
                    return formBox({ type: validateState, err: requestTokenError(state.err) })
            }
        } else {
            return buttonBox({
                type: state.type === "success" ? "success" : "initial",
            })
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
                                html`パスワードリセットのための<br />
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
                props.requestToken.clear()
                props.editable.open()
            }
        }
    }

    type FormContentType = "initial" | "valid" | "invalid" | "connecting" | "take-longtime"
    type FormContent = Readonly<{ type: FormContentType }> &
        Partial<{ err: readonly VNodeContent[] }>
    function formBox(state: FormContent): VNode {
        return form(
            box({
                title: "パスワードリセットトークン送信",
                body: [
                    h(InputLoginIdEntry, {
                        field: props.requestToken.loginId,
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
                        label: html`リセットトークン送信中 ${icon_spinner}`,
                    })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.requestToken.submit().then(() => {
                    props.editable.close()
                })
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
                props.requestToken.clear()
                props.editable.close()
            }
        }

        function message(): readonly VNode[] {
            if (state.err) {
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
                            html`${icon_spinner} リセットトークン送信中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "invalid":
                    return [fieldError(["正しく入力されていません"])]
            }
        }
    }
}

function requestTokenError(err: RequestResetTokenError): readonly VNodeContent[] {
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
