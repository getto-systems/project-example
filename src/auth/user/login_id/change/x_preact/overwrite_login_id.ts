import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons, fieldHelp_error } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField, validationMessage } from "../../../../../core/x_preact/design/form"

import { changeLoginIdError } from "./helper"
import { LoginIdField } from "../../input/x_preact/input"
import { EditButton } from "../../../../../core/x_preact/button/edit_button"
import { EditSuccessButton } from "../../../../../core/x_preact/button/edit_success_button"
import { ClearChangesButton } from "../../../../../core/x_preact/button/clear_changes_button"
import { ChangeButton } from "../../../../../core/x_preact/button/change_button"
import { CloseButton } from "../../../../../core/x_preact/button/close_button"

import { OverwriteLoginIdAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"

import { LoginId } from "../../kernel/data"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId }>
    editable: EditableBoardAction
    overwrite: OverwriteLoginIdAction
    onSuccess: { (loginId: LoginId): void }
}>
export function OverwriteLoginId(props: Props): VNode {
    const state = useApplicationAction(props.overwrite)
    const editableState = useApplicationAction(props.editable)
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
            props.overwrite.clear()
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
            props.overwrite.submit(props.user, onSuccess)

            function onSuccess(loginId: LoginId) {
                props.editable.close()
                props.onSuccess(loginId)
            }
        }
    }

    function clearButton(): VNode {
        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.overwrite.clear()
        }
    }
    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
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
                return [fieldHelp_error(changeLoginIdError(state.err))]
        }
    }
}
