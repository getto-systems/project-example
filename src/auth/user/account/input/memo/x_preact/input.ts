import { h, VNode } from "preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useApplicationAction } from "../../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { useEditableState } from "../../../../../../z_vendor/getto-application/board/editable/x_preact/hooks"

import {
    inputField,
    label_text_fill,
} from "../../../../../../z_vendor/getto-css/preact/design/form"

import { InputBoard } from "../../../../../../z_vendor/getto-application/board/input/x_preact/input"

import { textValidationError } from "../../../../../../z_lib/ui/validate/x_plain/error"

import { InputAuthUserMemoAction } from "../action"
import { EditableBoardAction } from "../../../../../../z_vendor/getto-application/board/editable/action"

import { AuthUserMemo } from "../../../kernel/data"

type Props = Readonly<{ field: InputAuthUserMemoAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        edit: Readonly<{
            data: Readonly<{ memo: AuthUserMemo }>
            editable: EditableBoardAction
        }>
    }>

export function AuthUserMemoField(props: Props): VNode {
    const validateState = useApplicationAction(props.field.validate)
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || "備考",
        help: props.help,
        label: label_text_fill,
        state: validateState.valid
            ? { type: "normal" }
            : { type: "error", notice: textValidationError(validateState.err) },
        body: editableState.isEditable
            ? h(InputBoard, { type: "text", input: props.field.input })
            : editableState.data.memo,
    })
}
