import { h, VNode } from "preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { useEditableState } from "../../../../../../z_vendor/getto-application/board/editable/x_preact/hooks"

import {
    inputField,
    label_text_fill,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { mapValidateState } from "../../../../../../z_lib/ui/input/field/x_preact/helper"

import { InputBoard } from "../../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { authUserMemo } from "../../../kernel/x_preact/field"

import { textValidationError } from "../../../../../../z_lib/ui/validate/x_plain/error"

import { AuthUserTextFieldAction } from "../action"
import { EditableBoardAction } from "../../../../../../z_vendor/getto-application/board/editable/action"

import { AuthUserTextField } from "../convert"

import { TypeAuthUser, AUTH_USER_ACCOUNT } from "../../../kernel/data"

type TextProps<K extends AuthUserTextField> = Readonly<{ field: AuthUserTextFieldAction<K> }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        edit: Readonly<{
            data: Readonly<{ [key in K]: TypeAuthUser<K> }>
            editable: EditableBoardAction
        }>
    }>

export function AuthUserMemoField(props: TextProps<"memo">): VNode {
    const validateState = useApplicationAction(props.field.validate)
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["memo"],
        help: props.help,
        label: label_text_fill,
        editableState,
        validateState: mapValidateState(validateState, textValidationError),
        body: editableState.isEditable
            ? h(InputBoard, { type: "text", input: props.field.input })
            : authUserMemo(editableState.data),
    })
}
