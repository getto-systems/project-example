import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, validationMessage } from "../../../../../common/x_preact/design/form"

import { changePasswordError } from "./helper"
import { PasswordField } from "../../input/x_preact/input"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"

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

    return box({
        form: true,
        title: "パスワード変更",
        ...(editableState.isEditable
            ? {
                  body: [
                      h(PasswordField, {
                          field: props.change.currentPassword,
                          title: "現在のパスワード",
                          help: ["変更前のパスワードを入力します"],
                          autocomplete: "current-password",
                      }),
                      h(PasswordField, {
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
                      ...validationMessage(validateState),
                      ...message(),
                      buttons({
                          right: closeButton(),
                      }),
                  ],
              }
            : {
                  body: editButton(),
              }),
    })

    function editButton(): VNode {
        if (state.type === "success") {
            return h(EditSuccessButton, { onClick })
        } else {
            return h(EditButton, { onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.change.clear()
            props.editable.open()
        }
    }

    function submitButton(): VNode {
        return h(ChangeButton, {
            isConnecting: state.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.submit(onSuccess)

            function onSuccess() {
                props.editable.close()
            }
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
                return []

            case "try":
                if (state.hasTakenLongtime) {
                    return [takeLongtimeField("変更")]
                }
                return []

            case "failed":
                return [fieldHelp_error(changePasswordError(state.err))]
        }
    }
}
