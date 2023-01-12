import { h, VNode } from "preact"

import { useEditableState } from "../../../../z_vendor/getto-application/board/editable/x_preact/hooks"

import { inputField, label } from "../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../common/x_preact/vnode"

import {
    SelectBoard,
    SelectBoardContent,
} from "../../../../z_vendor/getto-application/board/input/x_preact/select"

import { seasonLabel } from "../../kernel/helper"
import { seasonToString } from "../../kernel/convert"

import { SeasonFieldAction } from "../action"
import { EditableBoardAction } from "../../../../z_vendor/getto-application/board/editable/action"

import { Season } from "../../kernel/data"

type Props = Readonly<{
    field: SeasonFieldAction
    availableSeasons: readonly Season[]
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        edit: Readonly<{
            data: Readonly<{ season: Season }>
            editable: EditableBoardAction
        }>
    }>

export function SeasonField(props: Props): VNode {
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || "シーズン",
        help: props.help,
        label: label,
        editableState,
        body: editableState.isEditable
            ? h(SelectBoard, { input: props.field.input, options: options(props.availableSeasons) })
            : seasonLabel(editableState.data.season),
    })

    function options(seasons: readonly Season[]): readonly SelectBoardContent[] {
        return [
            { key: "", value: "", label: "今シーズン" },
            ...seasons.map((season) => ({
                key: seasonToString(season),
                value: seasonToString(season),
                label: seasonLabel(season),
            })),
        ]
    }
}
