import { h } from "preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/node"

import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"
import { useEditableState } from "../../../../../../common/util/board/editable/x_preact/hooks"

import {
    field,
    inputField,
    label_text_fill,
} from "../../../../../../z_vendor/getto-css/preact/design/form"
import { mapValidateBoardValue } from "../../../../../../common/util/input/field/x_preact/helper"

import { InputBoard } from "../../../../../../common/util/board/input/x_preact/input"

import { textValidationError } from "../../../../../../common/util/validate/x_plain/error"

import { LoginIdField } from "../action"
import { EditableBoardAction } from "../../../../../../common/util/board/editable/action"

import { LoginId } from "../../../kernel/data"
import { AUTH_USER_ACCOUNT } from "../../../../account/kernel/data"

type Props = Readonly<{ field: LoginIdField }> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
        edit: Readonly<{
            data: LoginId
            editable: EditableBoardAction
        }>
        autocomplete: string
    }>

export function AuthUserLoginIdField(props: Props): PreactNode {
    const validateState = useAtom(props.field.validate)
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["loginId"],
        help: props.help,
        label: label_text_fill,
        editableState,
        validateState: mapValidateBoardValue(validateState, textValidationError),
        body: editableState.isEditable
            ? h(InputBoard, {
                  type: "text",
                  input: props.field.input,
                  autocomplete: props.autocomplete,
              })
            : loginId(editableState.data),
    })
}

type StaticProps = Readonly<{
    data: Readonly<{ loginId: LoginId }>
}> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
    }>
export function AuthUserLoginIdStaticField(props: StaticProps): PreactNode {
    return field({
        title: props.title || AUTH_USER_ACCOUNT["loginId"],
        help: props.help,
        body: loginId(props.data.loginId),
    })
}

function loginId(data: LoginId): PreactContent {
    return data
}
