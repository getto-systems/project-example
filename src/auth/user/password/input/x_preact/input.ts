import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_password_fill,
} from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoardComponent } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { ValidateBoardFieldState } from "../../../../../z_vendor/getto-application/board/validate_field/action"

import { ValidatePasswordError } from "../data"
import { InputPasswordAction, ValidatePasswordState } from "../action"

type EntryProps = Readonly<{ field: InputPasswordAction }> &
    Partial<{ title: VNodeContent; help: readonly VNodeContent[] }>
export function InputPasswordEntry(resource: EntryProps): VNode {
    return h(InputPasswordComponent, {
        ...resource,
        state: useApplicationAction(resource.field.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: ValidatePasswordState
    }>
export function InputPasswordComponent(props: Props): VNode {
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
            body: h(InputBoardComponent, { type: "password", input: props.field.input }),
            help: [...help(), characterHelp()],
        }

        if (props.state.valid) {
            return field(content)
        } else {
            return field_error({
                ...content,
                notice: passwordValidationError(props.state),
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

function passwordValidationError(
    result: ValidateBoardFieldState<ValidatePasswordError>,
): readonly VNodeContent[] {
    if (result.valid) {
        return []
    }

    return result.err.map((err) => {
        switch (err.type) {
            case "empty":
                return ["パスワードを入力してください"]

            case "too-long":
                return [`パスワードが長すぎます(${err.maxLength}文字以内)`]
        }
    })
}
