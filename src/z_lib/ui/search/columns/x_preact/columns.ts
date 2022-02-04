import { h, VNode } from "preact"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"
import { tableViewColumns } from "../../../../../z_vendor/getto-css/preact/design/data"

import { VNodeContent, VNodeKey } from "../../../x_preact/common"

import {
    CheckboxBoardComponent,
    CheckboxBoardContent,
} from "../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import { SearchColumnsAction } from "../action"

export type SearchColumnContent = Readonly<{
    key: VNodeKey
    content: VNodeContent
}>

type Props = Readonly<{
    field: SearchColumnsAction
    columns: readonly SearchColumnContent[]
}> &
    Partial<{
        title: VNodeContent
        block: boolean
    }>
export function SearchColumnsComponent(props: Props): VNode {
    return field({
        title: title(),
        body: [
            tableViewColumns(
                h(CheckboxBoardComponent, {
                    input: props.field.input,
                    options: options(),
                    block: block(),
                }),
            ),
        ],
    })

    function title(): VNodeContent {
        if (props.title) {
            return props.title
        }
        return "表示する列"
    }
    function options(): readonly CheckboxBoardContent[] {
        return props.columns.map((column) => ({
            key: column.key,
            value: `${column.key}`,
            label: column.content,
        }))
    }
    function block(): boolean {
        if (props.block) {
            return props.block
        }
        return false
    }
}
