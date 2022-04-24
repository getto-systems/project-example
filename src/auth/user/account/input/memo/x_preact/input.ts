import { h, VNode } from "preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useEditableState } from "../../../../../../z_vendor/getto-application/board/editable/x_preact/hooks"

import { field, label_text_fill } from "../../../../../../z_vendor/getto-css/preact/design/form"

import { InputBoard } from "../../../../../../z_vendor/getto-application/board/input/x_preact/input"

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
    const editableState = useEditableState(props.edit)

    return label_text_fill(
        field({
            title: props.title || "備考",
            help: props.help,
            body: body(),
        }),
    )

    function body(): VNodeContent {
        if (!editableState.isEditable) {
            return editableState.data.memo
        }
        return h(InputBoard, {
            type: "text",
            input: props.field.input,
        })
    }
}
