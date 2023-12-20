import { h } from "preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"
import { useEditableState } from "../../../../../../common/util/board/editable/x_preact/hooks"

import {
    inputField,
    label_text_fill,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { mapValidateBoardValue } from "../../../../../../common/util/input/field/x_preact/helper"
import { textValidationError } from "../../../../../../common/util/validate/x_plain/error"
import { authUserMemo } from "../../../kernel/x_preact/field"
import { InputBoard } from "../../../../../../common/util/board/input/x_preact/input"

import { EditableBoardAction } from "../../../../../../common/util/board/editable/action"
import { AuthUserTextField } from "../action"

import { AuthUserTextFieldName } from "../convert"

import { TypeAuthUser, AUTH_USER_ACCOUNT } from "../../../kernel/data"

type FieldProps<A, T> = Readonly<{ field: A }> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
        edit: Readonly<{
            data: T
            editable: EditableBoardAction
        }>
    }>

type TextProps<K extends AuthUserTextFieldName> = FieldProps<
    AuthUserTextField<K>,
    Readonly<{ [key in K]: TypeAuthUser<K> }>
>

export function AuthUserMemoField(props: TextProps<"memo">): PreactNode {
    const validateState = useAtom(props.field.validate)
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["memo"],
        help: props.help,
        label: label_text_fill,
        editableState,
        validateState: mapValidateBoardValue(validateState, textValidationError),
        body: editableState.isEditable
            ? h(InputBoard, { type: "text", input: props.field.input })
            : authUserMemo(editableState.data),
    })
}
