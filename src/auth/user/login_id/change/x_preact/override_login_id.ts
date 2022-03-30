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

import { changeLoginIdError } from "./helper"
import { InputLoginIdEntry } from "../../input/x_preact/input"

import { OverrideLoginIdAction, OverrideLoginIdState } from "../action"
import { ValidateBoardState } from "../../../../../z_vendor/getto-application/board/validate_board/action"
import {
    EditableBoardAction,
    EditableBoardState,
} from "../../../../../z_vendor/getto-application/board/editable/action"

import { LoginId } from "../../input/data"

type EntryProps = Readonly<{
    user: Readonly<{ loginId: LoginId }>
    editable: EditableBoardAction
    override: OverrideLoginIdAction
}>
export function OverrideLoginIdEntry({ user, editable, override }: EntryProps): VNode {
    return h(OverrideLoginIdComponent, {
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
        state: OverrideLoginIdState
        editableState: EditableBoardState
        validateState: ValidateBoardState
    }>
export function OverrideLoginIdComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, editableState, validateState }: Props): VNode {
        if (editableState.isEditable) {
            switch (state.type) {
                case "initial-override-login-id":
                case "succeed-to-override-login-id":
                    return formBox({ type: validateState })

                case "try-to-override-login-id":
                    return formBox({ type: "connecting" })

                case "take-longtime-to-override-login-id":
                    return formBox({ type: "take-longtime" })

                case "failed-to-override-login-id":
                    return formBox({ type: validateState, err: changeLoginIdError(state.err) })
            }
        } else {
            return buttonBox({
                type: state.type === "succeed-to-override-login-id" ? "success" : "initial",
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
                        ? notice_success(["ログインIDを変更しました"])
                        : undefined,
            }
        }
        function openButton(): VNode {
            return button_send({ state: "normal", label: "変更", onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.override.clear()
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
                    h(InputLoginIdEntry, {
                        field: props.override.newLoginId,
                        title: "新しいログインID",
                        help: ["管理者権限でログインIDを上書きします"],
                        autocomplete: "username",
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
                        case "succeed-to-override-login-id":
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
                            html`${icon_spinner} ログインID変更中です`,
                            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                        ]),
                    ]

                case "invalid":
                    return [fieldError(["正しく入力されていません"])]
            }
        }
    }
}

const BOX_TITLE = "ログインID"

const LABEL_STATIC = html`変更 ${iconHtml(icon_save)}`
const LABEL_CONNECT = html`変更 ${iconHtml(icon_spinner)}`
const LABEL_CLEAR = "入力内容をクリア"

const EMPTY_CONTENT = html``
