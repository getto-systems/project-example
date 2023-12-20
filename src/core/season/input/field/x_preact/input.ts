import { PreactContent, PreactNode } from "../../../../../common/x_preact/node"
import { h } from "preact"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"
import { useEditableState } from "../../../../../common/util/board/editable/x_preact/hooks"

import { SelectBoard } from "../../../../../common/util/board/input/x_preact/select"

import { inputField, label } from "../../../../../z_vendor/getto-css/preact/design/form"

import { seasonLabel } from "../../../kernel/helper"
import { seasonToString } from "../../../kernel/convert"

import { EditableBoardAction } from "../../../../../common/util/board/editable/action"

import { DetectedSeason } from "../../../kernel/data"
import { SelectFieldBoard } from "../../../../../common/util/board/field/action"
import { selectOptions } from "../../../../../common/x_preact/design/select"

type Props = Readonly<{
    field: SelectFieldBoard<DetectedSeason>
}> &
    Partial<{
        title: PreactContent
        help: readonly PreactContent[]
        edit: Readonly<{
            data: Readonly<{ season: DetectedSeason }>
            editable: EditableBoardAction
        }>
    }>

export function SeasonField(props: Props): PreactNode {
    const editableState = useEditableState(props.edit)
    const options = useAtom(props.field.options)

    return inputField({
        title: props.title || "シーズン",
        help: props.help,
        label: label,
        editableState,
        body: editableState.isEditable
            ? h(SelectBoard, {
                  input: props.field.input,
                  options: selectOptions(options, (data) => ({
                      key: seasonToString(data),
                      value: seasonToString(data),
                      label: seasonLabel(data),
                  })),
              })
            : seasonLabel(editableState.data.season),
    })
}
