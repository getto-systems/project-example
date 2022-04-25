import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { inputField, label_text_fill } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { textValidationError } from "../../../../../z_lib/ui/validate/x_plain/error"

import { InputLoginIdAction } from "../action"

type Props = Readonly<{
    field: InputLoginIdAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function LoginIdField(props: Props): VNode {
    const validateState = useApplicationAction(props.field.validate)

    return inputField({
        title: props.title || "ログインID",
        help: props.help,
        label: label_text_fill,
        state: validateState.valid
            ? { type: "normal" }
            : { type: "error", notice: textValidationError(validateState.err) },
        body: h(InputBoard, {
            type: "text",
            input: props.field.input,
            autocomplete: props.autocomplete,
        }),
    })
}
