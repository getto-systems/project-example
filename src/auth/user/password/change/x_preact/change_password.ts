import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldError, form } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_success } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { iconHtml } from "../../../../../core/x_preact/design/icon"
import { icon_spinner } from "../../../../../x_content/icon"

import { changePasswordError } from "./helper"
import { InputPassword } from "../../input/x_preact/input"
import { ClearChangesButton } from "../../../../../core/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../core/x_preact/button/close_button"
import { ChangeButton } from "../../../../../core/x_preact/button/change_button"
import { EditButton } from "../../../../../core/x_preact/button/edit_button"

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
    const observeState = useApplicationAction(props.change.observe)

    const content = {
        title: "パスワード",
    }

    if (!editableState.isEditable) {
        return form(
            box({
                ...content,
                body: editButton(),
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

    function editButton(): VNode {
        return h(EditButton, { isSuccess: state.type === "success", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.clear()
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(ChangeButton, {
            isConnecting: state.type === "try" || state.type === "take-longtime",
            validateState,
            onClick,
        })

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
        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.clear()
        }
    }

    function closeButton(): VNode {
        return h(CloseButton, { onClick })

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
                        html`${iconHtml(icon_spinner)} 変更に時間がかかっています`,
                        html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
                    ]),
                ]

            case "failed":
                return [fieldError(changePasswordError(state.err))]
        }
    }
}
