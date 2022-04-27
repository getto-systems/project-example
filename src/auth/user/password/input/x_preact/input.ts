import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    inputField,
    label_password_fill,
} from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { textValidationError } from "../../../../../z_lib/ui/validate/x_plain/error"

import { InputPasswordAction } from "../action"
import { AUTH_USER_ACCOUNT } from "../../../account/kernel/data"

type Props = Readonly<{ field: InputPasswordAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function PasswordField(props: Props): VNode {
    const validateState = useApplicationAction(props.field.validate)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["password"],
        help: [...(props.help || []), characterHelp()],
        label: label_password_fill,
        state:
            validateState.type === "initial" || validateState.result.valid
                ? { type: "normal" }
                : { type: "error", notice: textValidationError(validateState.result.err) },
        body: h(InputBoard, {
            type: "password",
            input: props.field.input,
            autocomplete: props.autocomplete,
        }),
    })

    function characterHelp(): string {
        if (props.field.checkCharacter().multiByte) {
            return "(マルチバイト文字が含まれています)"
        } else {
            return ""
        }
    }
}
