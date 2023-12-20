import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../../../../../common/x_preact/node"

import { useAtom } from "../../../../../../../../z_vendor/getto-atom/x_preact/hooks"
import { useEditableState } from "../../../../../../../../common/util/board/editable/x_preact/hooks"

import {
    inputField,
    label_text_fill,
} from "../../../../../../../../z_vendor/getto-css/preact/design/form"
import { label_gray } from "../../../../../../../../z_vendor/getto-css/preact/design/highlight"
import { radioOptions } from "../../../../../../../../common/x_preact/design/radio"
import { textValidationError } from "../../../../../../../../common/util/validate/x_plain/error"
import { mapValidateBoardValue } from "../../../../../../../../common/util/input/field/x_preact/helper"
import { resetTokenDestinationTypeLabel } from "../../../kernel/x_preact/fields"

import { RadioBoard } from "../../../../../../../../common/util/board/input/x_preact/radio"
import { InputBoard } from "../../../../../../../../common/util/board/input/x_preact/input"

import { EditableBoardAction } from "../../../../../../../../common/util/board/editable/action"
import { ResetTokenDestinationField } from "../action"

import { AUTH_USER_ACCOUNT } from "../../../../../../account/kernel/data"
import { ResetTokenDestination } from "../../../kernel/data"
import { ValidateResetTokenDestinationError } from "../data"

type Props = Readonly<{
    field: ResetTokenDestinationField
}> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
        autocomplete: string
        edit: Readonly<{
            data: Readonly<{ resetTokenDestination: ResetTokenDestination }>
            editable: EditableBoardAction
        }>
    }>
export function ResetTokenDestinationField(props: Props): PreactNode {
    const type = useAtom(props.field.type.value)
    const editableState = useEditableState(props.edit)
    const validateState = useAtom(props.field.validate)
    const options = useAtom(props.field.type.options)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["resetTokenDestination"],
        help: props.help,
        label: label_text_fill,
        editableState,
        validateState: mapValidateBoardValue(validateState, validationError),
        body: editableState.isEditable
            ? [
                  h(RadioBoard, {
                      input: props.field.type.input,
                      name: "destinationType",
                      options: radioOptions(options, (type) => ({
                          key: type,
                          value: type,
                          label: resetTokenDestinationTypeLabel(type),
                      })),
                  }),
                  email(),
              ]
            : h(ResetTokenDestinationLabel, editableState.data),
    })

    function email(): PreactNode {
        switch (type) {
            case "email":
                return h(InputBoard, {
                    type: "email",
                    input: props.field.email.input,
                    autocomplete: props.autocomplete,
                })

            default:
                return html``
        }
    }
}

export function ResetTokenDestinationLabel({
    resetTokenDestination,
}: Readonly<{ resetTokenDestination: ResetTokenDestination }>): PreactNode {
    switch (resetTokenDestination.type) {
        case "none":
            return label_gray(resetTokenDestinationTypeLabel(resetTokenDestination.type))

        case "email":
            return html`${resetTokenDestination.email}`
    }
}

function validationError(err: ValidateResetTokenDestinationError): readonly PreactContent[] {
    switch (err.type) {
        case "type":
            return ["有効/無効を選択してください"]

        case "email":
            return textValidationError(err.err)
    }
}
