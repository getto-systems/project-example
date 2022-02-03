import { h, VNode } from "preact"
import { html } from "htm/preact"

import { field } from "../../../../../ui/vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../z_lib/ui/x_preact/common"

import { SelectBoardComponent } from "../../../../../ui/vendor/getto-application/board/input/x_preact/select"

import { seasonLabel } from "../../kernel/helper"

import { InputSeasonAction } from "../action"

import { Season } from "../../kernel/data"
import { seasonToBoardValue } from "../../kernel/convert"

type Props = Readonly<{
    field: InputSeasonAction
    title: VNodeContent
    seasons: Season[]
}>
export function InputSeasonComponent(props: Props): VNode {
    return field({
        title: props.title,
        body: h(SelectBoardComponent, {
            input: props.field.input,
            options: options(props.seasons),
        }),
    })

    function options(seasons: Season[]): VNode[] {
        return [
            html`<option value="">今シーズン</option>`,
            ...seasons.map((season) => {
                const label = `${seasonLabel(season)}`
                return html`<option value="${seasonToBoardValue(season)}">${label}</option>`
            }),
        ]
    }
}
