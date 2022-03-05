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
import { icon_save, icon_spinner } from "../../../../../core/x_preact/design/icon"

import { changePasswordError } from "./helper"
import { InputPasswordEntry } from "../../input/x_preact/input"

import { OverridePasswordAction, OverridePasswordState } from "../action"
import { ValidateBoardState } from "../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    EditableBoardAction,
    EditableBoardState,
} from "../../../../../z_vendor/getto-application/board/editable/action"

import { AuthUserAccountBasket } from "../../../account/kernel/data"

type EntryProps = Readonly<{
    user: AuthUserAccountBasket
    editable: EditableBoardAction
    override: OverridePasswordAction
}>
export function OverridePasswordEntry({ user, editable, override }: EntryProps): VNode {
    return h(OverridePasswordComponent, {
        user,
        editable,
        override,
        state: useApplicationAction(override),
        editableState: useApplicationAction(editable),
        validateState: useApplicationAction(override.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: OverridePasswordState
        editableState: EditableBoardState
        validateState: ValidateBoardState
    }>
export function OverridePasswordComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, editableState, validateState }: Props): VNode {
        if (editableState.isEditable) {
            switch (state.type) {
                case "initial-override-password":
                    return formBox({ type: validateState })

                case "try-to-override-password":
                    return formBox({ type: "connecting" })

                case "take-longtime-to-override-password":
                    return formBox({ type: "take-longtime" })

                case "succeed-to-override-password":
                    return buttonBox({ type: "success" })

                case "failed-to-override-password":
                    return formBox({ type: validateState, err: changePasswordError(state.err) })
            }
        } else {
            return buttonBox({ type: "initial" })
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
                        footer: notice_success(["パスワードを変更しました"]),
                    }
            }
        }
        function title() {
            return "パスワード変更"
        }
        function openButton(): VNode {
            return button_send({ state: "normal", label: "変更", onClick })

            function onClick(e: Event) {
                e.preventDefault()
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
                title: "パスワード変更",
                body: [
                    h(InputPasswordEntry, {
                        field: props.override.newPassword,
                        title: "新しいパスワード",
                        help: ["管理者権限でパスワードを上書きします"],
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
                props.override.submit(props.user).then((state) => {
                    switch (state.type) {
                        case "succeed-to-override-password":
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
                props.override.clear()
            }
        }
        function closeButton(): VNode {
            return button_undo({ label: "閉じる", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.override.clear()
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

const LABEL_STATIC = html`変更 ${icon_save}`
const LABEL_CONNECT = html`変更 ${icon_spinner}`
const LABEL_CLEAR = "入力内容をクリア"

const EMPTY_CONTENT = html``
