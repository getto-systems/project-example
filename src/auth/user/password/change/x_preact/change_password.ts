import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_disabled,
    button_send,
    button_undo,
    fieldError,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"
import { iconHtml } from "../../../../../core/x_preact/design/icon"
import { icon_save, icon_spinner } from "../../../../../x_content/icon"

import { changePasswordError } from "./helper"
import { InputPasswordEntry } from "../../input/x_preact/input"

import { ChangePasswordAction, ChangePasswordState } from "../action"
import { ValidateBoardState } from "../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    EditableBoardAction,
    EditableBoardState,
} from "../../../../../z_vendor/getto-application/board/editable/action"

type EntryProps = Readonly<{
    editable: EditableBoardAction
    change: ChangePasswordAction
}>
export function ChangePasswordEntry({ editable, change }: EntryProps): VNode {
    return h(ChangePasswordComponent, {
        editable,
        change,
        state: useApplicationAction(change),
        editableState: useApplicationAction(editable),
        validateState: useApplicationAction(change.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: ChangePasswordState
        editableState: EditableBoardState
        validateState: ValidateBoardState
    }>
export function ChangePasswordComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, editableState, validateState }: Props): VNode {
        if (editableState.isEditable) {
            switch (state.type) {
                case "initial-change-password":
                case "succeed-to-change-password":
                    return formBox({ type: validateState })

                case "try-to-change-password":
                    return formBox({ type: "connecting" })

                case "take-longtime-to-change-password":
                    return formBox({ type: "take-longtime" })

                case "failed-to-change-password":
                    return formBox({ type: validateState, err: changePasswordError(state.err) })
            }
        } else {
            return buttonBox({
                type: state.type === "succeed-to-change-password" ? "success" : "initial",
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
            return {
                title: BOX_TITLE,
                body: openButton(),
                footer:
                    state.type === "success"
                        ? notice_success(["パスワードを変更しました"])
                        : undefined,
            }
        }
        function openButton(): VNode {
            return button_send({ state: "normal", label: "変更", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.clear()
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
                title: BOX_TITLE,
                body: [
                    h(InputPasswordEntry, {
                        field: props.change.currentPassword,
                        title: "現在のパスワード",
                        help: ["変更前のパスワードを入力します"],
                        autocomplete: "current-password",
                    }),
                    h(InputPasswordEntry, {
                        field: props.change.newPassword,
                        title: "新しいパスワード",
                        help: ["今後はこのパスワードになります"],
                        autocomplete: "new-password",
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
            switch (state.type) {
                case "initial":
                    return button_send({ state: "normal", label: LABEL_STATIC, onClick })

                case "valid":
                    return button_send({ state: "confirm", label: LABEL_STATIC, onClick })

                case "invalid":
                    return button_disabled({ label: LABEL_STATIC })

                case "connecting":
                case "take-longtime":
                    return button_send({ state: "connect", label: LABEL_CONNECT })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.submit().then((state) => {
                    switch (state.type) {
                        case "succeed-to-change-password":
                            props.editable.close()
                    }
                })
            }
        }

        function clearButton(): VNode {
            switch (state.type) {
                case "initial":
                    return button_disabled({ label: LABEL_CLEAR })

                case "connecting":
                case "take-longtime":
                    return EMPTY_CONTENT

                case "invalid":
                case "valid":
                    return button_undo({ label: LABEL_CLEAR, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.change.clear()
            }
        }
        function closeButton(): VNode {
            return button_undo({ label: "閉じる", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.change.clear()
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
                            html`${icon_spinner} パスワード変更中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "invalid":
                    return [fieldError(["正しく入力されていません"])]
            }
        }
    }
}

const BOX_TITLE = "パスワード"

const LABEL_STATIC = html`変更 ${iconHtml(icon_save)}`
const LABEL_CONNECT = html`変更 ${iconHtml(icon_spinner)}`
const LABEL_CLEAR = "入力内容をクリア"

const EMPTY_CONTENT = html``
