import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { useEditableState } from "../../../../../../../z_vendor/getto-application/board/editable/x_preact/hooks"

import {
    field,
    field_error,
    label_text_fill,
} from "../../../../../../../z_vendor/getto-css/preact/design/form"
import { label_gray } from "../../../../../../../z_vendor/getto-css/preact/design/highlight"

import { VNodeContent } from "../../../../../../../z_lib/ui/x_preact/common"

import { InputBoard } from "../../../../../../../z_vendor/getto-application/board/input/x_preact/input"
import {
    RadioBoard,
    RadioBoardContent,
} from "../../../../../../../z_vendor/getto-application/board/input/x_preact/radio"

import { InputResetTokenDestinationAction } from "../action"
import { EditableBoardAction } from "../../../../../../../z_vendor/getto-application/board/editable/action"

import { ValidateResetTokenDestinationError } from "../data"
import { ResetTokenDestination } from "../../kernel/data"

type Props = Readonly<{
    field: InputResetTokenDestinationAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        autocomplete: string
        edit: Readonly<{
            data: Readonly<{ resetTokenDestination: ResetTokenDestination }>
            editable: EditableBoardAction
        }>
    }>
export function ResetTokenDestinationField(props: Props): VNode {
    const state = useApplicationAction(props.field)
    const editableState = useEditableState(props.edit)
    const validateState = useApplicationAction(props.field.validate)

    const content = {
        title: props.title || "パスワードリセット用Eメール",
        help: props.help,
        body: body(),
    }

    if (editableState.isEditable && !validateState.valid) {
        return field_error({ ...content, notice: validationError(validateState.err) })
    }
    return field(content)

    function body(): VNodeContent {
        if (!editableState.isEditable) {
            return h(ResetTokenDestinationLabel, editableState.data)
        }
        return [
            label_text_fill(
                h(RadioBoard, {
                    input: props.field.destinationType,
                    name: "destinationType",
                    options: [destinationRadio("email"), destinationRadio("none")],
                }),
            ),
            email(),
        ]

        function email(): VNode {
            switch (state.type) {
                case "none":
                    return html``

                case "email":
                    return h(InputBoard, {
                        type: "email",
                        input: props.field.email,
                        autocomplete: props.autocomplete,
                    })
            }
        }
        function destinationRadio(
            destinationType: ResetTokenDestination["type"],
        ): RadioBoardContent {
            return {
                key: destinationType,
                value: destinationType,
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

export function ResetTokenDestinationLabel({
    resetTokenDestination,
}: Readonly<{ resetTokenDestination: ResetTokenDestination }>): VNode {
    switch (resetTokenDestination.type) {
        case "none":
            return label_gray("無効")

        case "email":
            return html`${resetTokenDestination.email}`
    }
}

function validationError(
    err: readonly ValidateResetTokenDestinationError[],
): readonly VNodeContent[] {
    return err.map((err) => {
        switch (err.type) {
            case "invalid-type":
                return ["有効/無効を選択してください"]

            case "empty-email":
                return ["メールアドレスを入力してください"]

            case "invalid-email":
                return ["正しいメールアドレスを入力してください"]

            case "too-long-email":
                return [`メールアドレスが長すぎます(${err.maxLength}文字以内)`]
        }
    })
}