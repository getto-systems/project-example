import { h, VNode } from "preact"
import { VNodeContent } from "../../../../../common/x_preact/vnode"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { inputField, label_text_fill } from "../../../../../z_vendor/getto-css/preact/design/form"
import { mapValidateState } from "../../../../../common/util/input/field/x_preact/helper"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { textValidationError } from "../../../../../common/util/validate/x_plain/error"

import { LoginIdFieldAction } from "../action"
import { AUTH_USER_ACCOUNT } from "../../../account/kernel/data"

type Props = Readonly<{
    field: LoginIdFieldAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function LoginIdField(props: Props): VNode {
    const validateState = useApplicationState(props.field.validate.state)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["loginId"],
        help: props.help,
        label: label_text_fill,
        validateState: mapValidateState(validateState, textValidationError),
        body: h(InputBoard, {
            type: "text",
            input: props.field.input,
            autocomplete: props.autocomplete,
        }),
    })
}
