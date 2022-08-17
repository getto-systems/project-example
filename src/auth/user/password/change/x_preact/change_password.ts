import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidationMessage } from "../../../../../common/x_preact/design/form"

import { changePasswordError } from "./helper"
import { PasswordField } from "../../input/x_preact/input"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"

import { ChangePasswordAction } from "../action"

type Props = Readonly<{
    change: ChangePasswordAction
}>
export function ChangePassword(props: Props): VNode {
    const editableState = useApplicationState(props.change.editable.state)

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
                      buttons({ left: h(Submit, {}), right: h(Clear, {}) }),
                      h(ValidationMessage, props.change.validate),
                      h(Message, {}),
                      buttons({ right: h(Close, {}) }),
                  ],
              }
            : {
                  body: h(Edit, {}),
              }),
    })

    function Edit(_props: unknown): VNode {
        const changeState = useApplicationState(props.change.state)

        if (changeState.type === "success") {
            return h(EditSuccessButton, { onClick })
        } else {
            return h(EditButton, { onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.change.edit()
        }
    }

    function Submit(_props: unknown): VNode {
        const changeState = useApplicationState(props.change.state)
        const validateState = useApplicationState(props.change.validate.state)
        const observeState = useApplicationState(props.change.observe.state)

        return h(ChangeButton, {
            isConnecting: changeState.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.submit()
        }
    }

    function Clear(_props: unknown): VNode {
        const observeState = useApplicationState(props.change.observe.state)

        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.clear()
        }
    }

    function Close(_props: unknown): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.change.editable.close()
        }
    }

    function Message(_props: unknown): VNode {
        const changeState = useApplicationState(props.change.state)

        switch (changeState.type) {
            case "initial":
            case "success":
                return html``

            case "try":
                if (changeState.hasTakenLongtime) {
                    return takeLongtimeField("変更")
                }
                return html``

            case "failed":
                return fieldHelp_error(changePasswordError(changeState.err))
        }
    }
}
