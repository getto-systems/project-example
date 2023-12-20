import { h } from "preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/node"

import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"
import { useEditableState } from "../../../../../../common/util/board/editable/x_preact/hooks"

import {
    inputField,
    label_text_fill,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { mapValidateBoardValue } from "../../../../../../common/util/input/field/x_preact/helper"
import { textValidationError } from "../../../../../../common/util/validate/x_plain/error"

import { InputBoard } from "../../../../../../common/util/board/input/x_preact/input"
import { PasswordField } from "../action"

import { EditableBoardAction } from "../../../../../../common/util/board/editable/action"

import { Password } from "../data"
import { AUTH_USER_ACCOUNT } from "../../../../account/kernel/data"

type Props = Readonly<{ field: PasswordField }> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
        edit: Readonly<{
            data: Readonly<{ password: Password }>
            editable: EditableBoardAction
        }>
        autocomplete: string
    }>

export function AuthUserPasswordField(props: Props): PreactNode {
    const validateState = useAtom(props.field.validate)
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["password"],
        help: props.help,
        label: label_text_fill,
        editableState,
        validateState: mapValidateBoardValue(validateState, textValidationError),
        body: editableState.isEditable
            ? h(InputBoard, {
                  type: "password",
                  input: props.field.input,
                  autocomplete: props.autocomplete,
              })
            : authUserPassword(editableState.data),
    })
}

export function authUserPassword(data: Readonly<{ password: Password }>): PreactContent {
    return data.password
}
