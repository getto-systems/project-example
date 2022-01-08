import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_text_fill,
} from "../../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoardComponent } from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/input"

import { ValidateBoardFieldState } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/action"
import { InputLoginIDAction, ValidateLoginIDState } from "../action"

import { loginIDLabel, ValidateLoginIDError } from "../data"

type EntryProps = Readonly<{ field: InputLoginIDAction }> &
    Partial<{ title: VNodeContent; help: VNodeContent[] }>
export function InputLoginIDEntry(resource: EntryProps): VNode {
    return h(InputLoginIDComponent, {
        ...resource,
        state: useApplicationAction(resource.field.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: ValidateLoginIDState
    }>

export function InputLoginIDComponent(props: Props): VNode {
    return label_text_fill(content())

    function content() {
        const content = {
            title: title(),
            body: h(InputBoardComponent, { type: "text", input: props.field.input }),
            help: help(),
        }

        if (props.state.valid) {
            return field(content)
        } else {
            return field_error({ ...content, notice: loginIDValidationError(props.state) })
        }
    }
    function title(): VNodeContent {
        if (props.title) {
            return props.title
        }
        return loginIDLabel
    }
    function help(): VNodeContent[] {
        if (props.help) {
            return props.help
        }
        return []
    }
}

function loginIDValidationError(
    result: ValidateBoardFieldState<ValidateLoginIDError>,
): VNodeContent[] {
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
