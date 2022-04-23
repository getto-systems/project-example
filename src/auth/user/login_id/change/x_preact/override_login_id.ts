import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    fieldHelp_error,
    form,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box } from "../../../../../z_vendor/getto-css/preact/design/box"
import { takeLongtimeField } from "../../../../../core/x_preact/design/form"

import { changeLoginIdError } from "./helper"
import { LoginIdField } from "../../input/x_preact/input"
import { EditButton } from "../../../../../core/x_preact/button/edit_button"
import { ClearChangesButton } from "../../../../../core/x_preact/button/clear_changes_button"
import { ChangeButton } from "../../../../../core/x_preact/button/change_button"
import { CloseButton } from "../../../../../core/x_preact/button/close_button"

import { OverrideLoginIdAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"

import { LoginId } from "../../kernel/data"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId }>
    editable: EditableBoardAction
    override: OverrideLoginIdAction
    onSuccess: { (loginId: LoginId): void }
}>
export function OverrideLoginId(props: Props): VNode {
    const state = useApplicationAction(props.override)
    const editableState = useApplicationAction(props.editable)
    const validateState = useApplicationAction(props.override.validate)
    const observeState = useApplicationAction(props.override.observe)

    return form(
        box({
            title: "ログインID",
            ...(editableState.isEditable
                ? {
                      body: h(LoginIdField, {
                          field: props.override.newLoginId,
                          title: "新しいログインID",
                          help: ["管理者権限でログインIDを上書きします"],
                          autocomplete: "username",
                      }),
                      footer: [
                          buttons({
                              left: submitButton(),
                              right: clearButton(),
                          }),
                          ...validationMessage(),
                          ...message(),
                          buttons({
                              right: closeButton(),
                          }),
                      ],
                  }
                : { body: editButton() }),
        }),
    )

    function editButton(): VNode {
        return h(EditButton, { isSuccess: state.type === "success", onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.override.clear()
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
            props.override.submit(props.user).then((state) => {
                switch (state.type) {
                    case "success":
                        props.editable.close()
                        props.onSuccess(state.loginId)
                }
            })
        }
    }

    function clearButton(): VNode {
        return h(ClearChangesButton, { observeState, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.override.clear()
        }
    }
    function closeButton(): VNode {
        return h(CloseButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.editable.close()
        }
    }

    function validationMessage(): readonly VNode[] {
        switch (validateState) {
            case "initial":
            case "valid":
                return []

            case "invalid":
                return [fieldHelp_error(["正しく入力されていません"])]
        }
    }
    function message(): readonly VNode[] {
        switch (state.type) {
            case "initial":
            case "success":
            case "try":
                return []

            case "take-longtime":
                return [takeLongtimeField("変更")]

            case "failed":
                return [fieldHelp_error(changeLoginIdError(state.err))]
        }
    }
}