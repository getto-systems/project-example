import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidationMessage } from "../../../../../common/x_preact/design/form"

import { PasswordField } from "../../input/x_preact/input"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"

import { changePasswordError } from "./helper"

import { OverwritePasswordAction } from "../action"

type Props = Readonly<{
    overwrite: OverwritePasswordAction
}>
export function OverwritePassword(props: Props): VNode {
    const editableState = useApplicationState(props.overwrite.editable.state)

    return box({
        form: true,
        title: "パスワード変更",
        ...(editableState.isEditable
            ? {
                  body: [
                      h(PasswordField, {
                          field: props.overwrite.newPassword,
                          title: "新しいパスワード",
                          help: ["管理者権限でパスワードを上書きします"],
                          autocomplete: "new-password",
                      }),
                  ],
                  footer: [
                      buttons({ left: h(Submit, {}), right: h(Clear, {}) }),
                      h(ValidationMessage, props.overwrite.validate),
                      h(Message, {}),
                      buttons({ right: h(Close, {}) }),
                  ],
              }
            : {
                  body: h(Edit, {}),
              }),
    })

    function Edit(_props: unknown): VNode {
        const overwriteState = useApplicationState(props.overwrite.state)

        if (overwriteState.type === "success") {
            return h(EditSuccessButton, { onClick })
        } else {
            return h(EditButton, { onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.editable.open()
        }
    }

    function Submit(_props: unknown): VNode {
        const overwriteState = useApplicationState(props.overwrite.state)
        const validateState = useApplicationState(props.overwrite.validate.state)
        const observeState = useApplicationState(props.overwrite.observe.state)

        return h(ChangeButton, {
            isConnecting: overwriteState.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.submit()
        }
    }

    function Clear(_props: unknown): VNode {
        const observeState = useApplicationState(props.overwrite.observe.state)

        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.reset()
        }
    }
    function Close(_props: unknown): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.editable.close()
        }
    }

    function Message(_props: unknown): VNode {
        const overwriteState = useApplicationState(props.overwrite.state)

        switch (overwriteState.type) {
            case "initial":
            case "success":
                return html``

            case "try":
                if (overwriteState.hasTakenLongtime) {
                    return takeLongtimeField("変更")
                }
                return html``

            case "failed":
                return fieldHelp_error(changePasswordError(overwriteState.err))
        }
    }
}
