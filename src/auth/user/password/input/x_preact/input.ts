import { h, VNode } from "preact"
import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    inputField,
    label_password_fill,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { mapValidateState } from "../../../../../z_lib/ui/input/field/x_preact/helper"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { textValidationError } from "../../../../../z_lib/ui/validate/x_plain/error"

import { PasswordFieldAction } from "../action"

import { AUTH_USER_ACCOUNT } from "../../../account/kernel/data"

type Props = Readonly<{ field: PasswordFieldAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function PasswordField(props: Props): VNode {
    const validateState = useApplicationState(props.field.validate.state)
    const characterState = useApplicationState(props.field.character.state)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["password"],
        help: [...(props.help || []), characterHelp()],
        label: label_password_fill,
        validateState: mapValidateState(validateState, textValidationError),
        body: h(InputBoard, {
            type: "password",
            input: props.field.input,
            autocomplete: props.autocomplete,
        }),
    })

    function characterHelp(): string {
        if (characterState.multiByte) {
            return "(マルチバイト文字が含まれています)"
        } else {
            return ""
        }
    }
}
