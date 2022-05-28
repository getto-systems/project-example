import { h, VNode } from "preact"

import { field } from "../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"

import {
    SelectBoard,
    SelectBoardContent,
} from "../../../../z_vendor/getto-application/board/input/x_preact/select"

import { seasonLabel } from "../../kernel/helper"
import { seasonToString } from "../../kernel/convert"

import { InputSeasonAction } from "../action"

import { Season } from "../../kernel/data"

type Props = Readonly<{
    field: InputSeasonAction
    title: VNodeContent
    seasons: readonly Season[]
}>
export function InputSeason(props: Props): VNode {
    return field({
        title: props.title,
        body: h(SelectBoard, {
            input: props.field.input,
            options: options(props.seasons),
        }),
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
