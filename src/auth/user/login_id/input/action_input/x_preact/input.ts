import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_text_fill,
} from "../../../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../../example/x_preact/design/common"

import { InputBoardComponent } from "../../../../../../../ui/vendor/getto-application/board/action_input/x_preact/input"

import { InputLoginIDResource, InputLoginIDResourceState } from "../resource"

import { ValidateBoardFieldState } from "../../../../../../../ui/vendor/getto-application/board/action_validate_field/action"

import { ValidateLoginIDError } from "../../data"

type InputLoginIDOptions =
    | Readonly<{ title: VNodeContent; help: VNodeContent[] }>
    | Readonly<{ title: VNodeContent }>
    | Readonly<{ help: VNodeContent[] }>
    | {
          /* no props */
      }

type Resource = InputLoginIDResource & InputLoginIDOptions
export function InputLoginIDEntry(resource: Resource): VNode {
    return h(InputLoginIDComponent, {
        ...resource,
        state: useApplicationAction(resource.field.validate),
    })
}

type Props = Resource & InputLoginIDResourceState
export function InputLoginIDComponent(props: Props): VNode {
    return label_text_fill(content())

    function content() {
        const content = {
            title: "ログインID",
            body: h(InputBoardComponent, { type: "text", input: props.field.input }),
            help: help(),
        }

        if (props.state.valid) {
            return field(content)
        } else {
            return field_error({ ...content, notice: loginIDValidationError(props.state) })
        }
    }
    function help(): VNodeContent[] {
        if ("help" in props) {
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
