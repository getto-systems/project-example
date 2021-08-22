import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_password_fill,
} from "../../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../example/_ui/x_preact/design/common"

import { InputBoardComponent } from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/input"

import { ValidateBoardFieldState } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/action"
import { InputPasswordResource, InputPasswordResourceState } from "../resource"

import { ValidatePasswordError } from "../../data"

type Resource = InputPasswordResource & Readonly<{ help: VNodeContent[] }>
export function InputPasswordEntry(resource: Resource): VNode {
    return h(InputPasswordComponent, {
        ...resource,
        state: useApplicationAction(resource.field.validate),
    })
}

type Props = Resource & InputPasswordResourceState
export function InputPasswordComponent(props: Props): VNode {
    return label_password_fill(content())

    function content() {
        const content = {
            title: "パスワード",
            body: h(InputBoardComponent, { type: "password", input: props.field.input }),
            help: [...props.help, characterHelp()],
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
): VNodeContent[] {
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
