import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_text_fill,
} from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoardComponent } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { ValidateBoardFieldState } from "../../../../../z_vendor/getto-application/board/validate_field/action"
import { InputLoginIdAction, ValidateLoginIdState } from "../action"

import { ValidateLoginIdError } from "../data"

type EntryProps = Readonly<{ field: InputLoginIdAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function InputLoginIdEntry(resource: EntryProps): VNode {
    return h(InputLoginIdComponent, {
        ...resource,
        state: useApplicationAction(resource.field.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: ValidateLoginIdState
    }>

export function InputLoginIdComponent(props: Props): VNode {
    return label_text_fill(content())

    function content() {
        const content = {
            title: title(),
            body: h(InputBoardComponent, {
                type: "text",
                input: props.field.input,
                autocomplete: props.autocomplete,
            }),
            help: help(),
        }

        if (props.state.valid) {
            return field(content)
        } else {
            return field_error({ ...content, notice: loginIdValidationError(props.state) })
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

function loginIdValidationError(
    result: ValidateBoardFieldState<ValidateLoginIdError>,
): readonly VNodeContent[] {
    if (result.valid) {
        return []
    }

    return result.err.map((err) => {
        switch (err.type) {
            case "empty":
                return ["ログインIDを入力してください"]

            case "too-long":
                return [`ログインIDが長すぎます(${err.maxLength}文字以内)`]
        }
    })
}
