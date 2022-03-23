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
import {
    EditableBoardAction,
    EditableBoardState,
} from "../../../../../z_vendor/getto-application/board/editable/action"

import { toBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/convert"

import { ResetTokenDestination, ValidateResetTokenDestinationError } from "../data"
import { AuthUserAccountBasket } from "../../kernel/data"
import { label_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

type EntryProps = Readonly<{
    user: AuthUserAccountBasket
    editable: EditableBoardAction
    field: InputResetTokenDestinationAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
    }>
export function InputResetTokenDestinationEntry(resource: EntryProps): VNode {
    return h(InputResetTokenDestinationComponent, {
        ...resource,
        state: useApplicationAction(resource.field),
        editableState: useApplicationAction(resource.editable),
        validateState: useApplicationAction(resource.field.validate),
    })
}

type Props = EntryProps &
    Readonly<{
        state: InputResetTokenDestinationState
        editableState: EditableBoardState
        validateState: ValidateResetTokenDestinationState
    }>

export function InputResetTokenDestinationComponent(props: Props): VNode {
    const content = {
        title: props.title || "パスワードリセット用Eメール",
        help: props.help,
        body: body(),
    }

    if (props.editableState.isEditable && !props.validateState.valid) {
        return field_error({ ...content, notice: emailValidationError(props.validateState) })
    }
    return field(content)

    function body(): VNodeContent {
        if (!props.editableState.isEditable) {
            switch (props.user.resetTokenDestination.type) {
                case "none":
                    return label_gray("無効")

                case "email":
                    return props.user.resetTokenDestination.email
            }
        }
        return [
            label_text_fill(
                h(RadioBoardComponent, {
                    input: props.field.destinationType,
                    name: "destinationType",
                    options: [destinationRadio("email"), destinationRadio("none")],
                }),
            ),
            email(),
        ]

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
                label: (() => {
                    switch (destinationType) {
                        case "none":
                            return "無効"

                        case "email":
                            return "有効"
                    }
                })(),
            }
        }
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
