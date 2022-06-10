import { h, VNode } from "preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { useEditableState } from "../../../../../../z_vendor/getto-application/board/editable/x_preact/hooks"

import {
    inputField,
    label,
    label_text_fill,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { mapValidateState } from "../../../../../../z_lib/ui/input/field/x_preact/helper"
import { checkboxOptions } from "../../../../../../common/x_preact/design/checkbox"

import { InputBoard } from "../../../../../../z_vendor/getto-application/board/input/x_preact/input"
import { CheckboxBoard } from "../../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import {
    authRoleCheckboxContent,
    authUserGrantedRoles,
    authUserMemo,
} from "../../../kernel/x_preact/field"

import { textValidationError } from "../../../../../../z_lib/ui/validate/x_plain/error"

import { AuthUserTextFieldAction, AuthUserGrantedRolesFieldAction } from "../action"
import { EditableBoardAction } from "../../../../../../z_vendor/getto-application/board/editable/action"

import { AuthUserTextField } from "../convert"

import { TypeAuthUser, AUTH_USER_ACCOUNT } from "../../../kernel/data"
import { AuthRole } from "../../../../kernel/data"

type FieldProps<A, T> = Readonly<{ field: A }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        edit: Readonly<{
            data: T
            editable: EditableBoardAction
        }>
    }>

type TextProps<K extends AuthUserTextField> = FieldProps<
    AuthUserTextFieldAction<K>,
    Readonly<{ [key in K]: TypeAuthUser<K> }>
>

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

export function AuthUserGrantedRolesField(
    props: FieldProps<
        AuthUserGrantedRolesFieldAction,
        Readonly<{ grantedRoles: readonly AuthRole[] }>
    >,
): VNode {
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["grantedRoles"],
        help: props.help,
        label: label,
        editableState,
        body: editableState.isEditable
            ? h(CheckboxBoard, {
                  input: props.field.input,
                  options: checkboxOptions(props.field.options(), authRoleCheckboxContent),
              })
            : authUserGrantedRoles(editableState.data),
    })
}
