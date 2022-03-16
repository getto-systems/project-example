import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    field,
    field_error,
    label_text_fill,
} from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputBoardComponent } from "../../../../../z_vendor/getto-application/board/input/x_preact/input"
import {
    RadioBoardComponent,
    RadioBoardContent,
} from "../../../../../z_vendor/getto-application/board/input/x_preact/radio"

import { ValidateBoardFieldState } from "../../../../../z_vendor/getto-application/board/validate_field/action"
import {
    InputResetTokenDestinationAction,
    InputResetTokenDestinationState,
    ValidateResetTokenDestinationState,
} from "../action"

import { toBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/convert"

import { ResetTokenDestination, ValidateResetTokenDestinationError } from "../data"

type EntryProps = Readonly<{ field: InputResetTokenDestinationAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function InputResetTokenDestinationEntry(resource: EntryProps): VNode {
    return h(InputResetTokenDestinationComponent, {
        ...resource,
        state: useApplicationAction(resource.field),
        validateState: useApplicationAction(resource.field.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: InputResetTokenDestinationState
        validateState: ValidateResetTokenDestinationState
    }>

export function InputResetTokenDestinationComponent(props: Props): VNode {
    return content()

    function content() {
        const content = {
            title: title(),
            body: [
                label_text_fill(
                    h(RadioBoardComponent, {
                        input: props.field.destinationType,
                        name: "destinationType",
                        options: [destinationRadio("email"), destinationRadio("none")],
                    }),
                ),
                email(),
            ],
            help: help(),
        }

        if (props.validateState.valid) {
            return field(content)
        } else {
            return field_error({ ...content, notice: emailValidationError(props.validateState) })
        }

        function email(): VNode {
            switch (props.state.type) {
                case "none":
                    return EMPTY_CONTENT

                case "email":
                    return h(InputBoardComponent, {
                        type: "email",
                        input: props.field.input,
                        autocomplete: props.autocomplete,
                    })
            }
        }
        function destinationRadio(
            destinationType: ResetTokenDestination["type"],
        ): RadioBoardContent {
            return {
                key: destinationType,
                value: toBoardValue(destinationType),
                label: label(destinationType),
            }
        }
        function label(destinationType: ResetTokenDestination["type"]): VNodeContent {
            switch (destinationType) {
                case "none":
                    return "無効"

                case "email":
                    return "有効"
            }
        }
    }
    function title(): VNodeContent {
        if (props.title) {
            return props.title
        }
        // TODO 一覧のカラムと一緒にしたい
        return "パスワードリセット用Eメール"
    }
    function help(): readonly VNodeContent[] {
        if (props.help) {
            return props.help
        }
        return []
    }
}

function emailValidationError(
    result: ValidateBoardFieldState<ValidateResetTokenDestinationError>,
): readonly VNodeContent[] {
    if (result.valid) {
        return []
    }

    return result.err.map((err) => {
        switch (err.type) {
            case "empty-email":
                return ["メールアドレスを入力してください"]

            case "invalid-email":
                return ["正しいメールアドレスを入力してください"]

            case "too-long-email":
                return [`メールアドレスが長すぎます(${err.maxLength}文字以内)`]
        }
    })
}

const EMPTY_CONTENT = html``
