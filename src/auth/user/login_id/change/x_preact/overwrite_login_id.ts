import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, validationMessage } from "../../../../../common/x_preact/design/form"

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
    const state = useApplicationAction(props.overwrite)
    const editableState = useApplicationAction(props.overwrite.editable)
    const validateState = useApplicationAction(props.overwrite.validate)
    const observeState = useApplicationAction(props.overwrite.observe)

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
            : { body: editButton() }),
    })

    function editButton(): VNode {
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

    function submitButton(): VNode {
        return h(ChangeButton, {
            isConnecting: state.type === "try",
            validateState,
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.submit()
        }
    }

    function clearButton(): VNode {
        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.reset()
        }
    }
    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.editable.close()
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
                return [fieldHelp_error(changeLoginIdError(state.err))]
        }
    }
}
