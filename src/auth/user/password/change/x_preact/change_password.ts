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

import { iconHtml } from "../../../../../core/x_preact/design/icon"
import { icon_save, icon_spinner } from "../../../../../x_content/icon"

import { changePasswordError } from "./helper"
import { InputPassword } from "../../input/x_preact/input"

import { ChangePasswordAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"

type Props = Readonly<{
    editable: EditableBoardAction
    change: ChangePasswordAction
}>
export function ChangePassword(props: Props): VNode {
    const state = useApplicationAction(props.change)
    const editableState = useApplicationAction(props.editable)
    const validateState = useApplicationAction(props.change.validate)

    const content = {
        title: "パスワード",
    }

    if (!editableState.isEditable) {
        return form(
            box({
                ...content,
                body: openButton(),
                footer:
                    state.type === "success"
                        ? notice_success(["パスワードを変更しました"])
                        : undefined,
            }),
        )
    }

    return form(
        box({
            ...content,
            body: [
                h(InputPassword, {
                    field: props.change.currentPassword,
                    title: "現在のパスワード",
                    help: ["変更前のパスワードを入力します"],
                    autocomplete: "current-password",
                }),
                h(InputPassword, {
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

    function openButton(): VNode {
        return button_send({ state: "normal", label: "変更", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.clear()
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        switch (state.type) {
            case "initial":
            case "success":
            case "failed":
                switch (validateState) {
                    case "initial":
                        return button_send({ state: "normal", label: LABEL_STATIC, onClick })

                    case "valid":
                        return button_send({ state: "confirm", label: LABEL_STATIC, onClick })

                    case "invalid":
                        return button_disabled({ label: LABEL_STATIC })
                }
                break

            case "try":
            case "take-longtime":
                return button_send({ state: "connect", label: LABEL_CONNECT })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.change.submit().then((state) => {
                switch (state.type) {
                    case "success":
                        props.editable.close()
                }
            })
        }
    }

    function clearButton(): VNode {
        switch (state.type) {
            case "initial":
            case "success":
            case "failed":
                switch (validateState) {
                    case "initial":
                        return button_disabled({ label: LABEL_CLEAR })

                    case "invalid":
                    case "valid":
                        return button_undo({ label: LABEL_CLEAR, onClick })
                }
                break

            case "try":
            case "take-longtime":
                return EMPTY_CONTENT
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
        switch (state.type) {
            case "initial":
            case "success":
                switch (validateState) {
                    case "initial":
                    case "valid":
                        return []

                    case "invalid":
                        return [fieldError(["正しく入力されていません"])]
                }
                break

            case "try":
                return []

            case "take-longtime":
                return [
                    fieldError([
                        html`${icon_spinner} パスワード変更中です`,
                        html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                    ]),
                ]

            case "failed":
                return [fieldError(changePasswordError(state.err))]
        }
    }
}

const LABEL_STATIC = html`変更 ${iconHtml(icon_save)}`
const LABEL_CONNECT = html`変更 ${iconHtml(icon_spinner)}`
const LABEL_CLEAR = "入力内容をクリア"

const EMPTY_CONTENT = html``
