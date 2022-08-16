import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, ValidationMessage } from "../../../../../common/x_preact/design/form"

import { changeLoginIdError } from "./helper"
import { LoginIdField } from "../../input/x_preact/field"
import { EditButton } from "../../../../../common/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../common/x_preact/button/edit_success_button"
import { ClearChangesButton } from "../../../../../common/x_preact/button/clear_changes_button"
import { ChangeButton } from "../../../../../common/x_preact/button/change_button"
import { CloseButton } from "../../../../../common/x_preact/button/close_button"

import { OverwriteLoginIdAction } from "../action"

type Props = Readonly<{
    overwrite: OverwriteLoginIdAction
}>
export function OverwriteLoginId(props: Props): VNode {
    const editableState = useApplicationState(props.overwrite.editable.state)

    return box({
        form: true,
        title: "ログインID変更",
        ...(editableState.isEditable
            ? {
                  body: h(LoginIdField, {
                      field: props.overwrite.newLoginId,
                      title: "新しいログインID",
                      help: ["管理者権限でログインIDを上書きします"],
                      autocomplete: "username",
                  }),
                  footer: [
                      buttons({ left: h(Submit, {}), right: h(Clear, {}) }),
                      h(ValidationMessage, props.overwrite.validate),
                      h(Message, {}),
                      buttons({ right: h(Close, {}) }),
                  ],
              }
            : { body: h(Edit, {}) }),
    })

    function Edit(_props: unknown): VNode {
        const state = useApplicationState(props.overwrite.state)
        if (state.type === "success") {
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
                return fieldHelp_error(changeLoginIdError(overwriteState.err))
        }
    }
}
