import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_text_fill,
} from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { InputLoginIdAction } from "../action"

import { ValidateLoginIdError } from "../data"

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

    // TODO 整理したい
    return label_text_fill(content())

    function content() {
        const content = {
            title: props.title || "ログインID",
            body: h(InputBoard, {
                type: "text",
                input: props.field.input,
                autocomplete: props.autocomplete,
            }),
            help: props.help,
        }

        if (validateState.valid) {
            return field(content)
        } else {
            return field_error({ ...content, notice: loginIdValidationError(validateState.err) })
        }
    }
}

function loginIdValidationError(err: readonly ValidateLoginIdError[]): readonly VNodeContent[] {
    return err.map((err) => {
        switch (err.type) {
            case "empty":
                return ["ログインIDを入力してください"]

            case "too-long":
                return [`ログインIDが長すぎます(${err.maxLength}文字以内)`]
        }
    })
}
