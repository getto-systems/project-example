import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_password_fill,
} from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoard } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { InputPasswordAction } from "../action"

import { ValidatePasswordError } from "../data"

type Props = Readonly<{ field: InputPasswordAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function InputPassword(props: Props): VNode {
    const state = useApplicationAction(props.field.validate)

    return label_password_fill(content())

    function title() {
        if (props.title) {
            return props.title
        }
        return "パスワード"
    }
    function content() {
        const content = {
            title: title(),
            body: h(InputBoard, {
                type: "password",
                input: props.field.input,
                autocomplete: props.autocomplete,
            }),
            help: [...help(), characterHelp()],
        }

        if (state.valid) {
            return field(content)
        } else {
            return field_error({
                ...content,
                notice: passwordValidationError(state.err),
            })
        }
    }
    function help(): readonly VNodeContent[] {
        if (props.help) {
            return props.help
        }
        return []
    }

    function characterHelp(): string {
        if (props.field.checkCharacter().multiByte) {
            return "(マルチバイト文字が含まれています)"
        } else {
            return ""
        }
    }
}

function passwordValidationError(err: readonly ValidatePasswordError[]): readonly VNodeContent[] {
    return err.map((err) => {
        switch (err.type) {
            case "empty":
                return ["パスワードを入力してください"]

            case "too-long":
                return [`パスワードが長すぎます(${err.maxLength}文字以内)`]
        }
    })
}
