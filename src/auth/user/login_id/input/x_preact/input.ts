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

type Props = Readonly<{ field: InputLoginIdAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function InputLoginId(props: Props): VNode {
    const validateState = useApplicationAction(props.field.validate)
    return label_text_fill(content())

    function content() {
        const content = {
            title: title(),
            body: h(InputBoard, {
                type: "text",
                input: props.field.input,
                autocomplete: props.autocomplete,
            }),
            help: help(),
        }

        if (validateState.valid) {
            return field(content)
        } else {
            return field_error({ ...content, notice: loginIdValidationError(validateState.err) })
        }
    }
    function title(): VNodeContent {
        if (props.title) {
            return props.title
        }
        return "ログインID"
    }
    function help(): readonly VNodeContent[] {
        if (props.help) {
            return props.help
        }
        return []
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
