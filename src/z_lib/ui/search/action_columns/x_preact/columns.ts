import { h, VNode } from "preact"

import { field } from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { tableViewColumns } from "../../../../../../ui/vendor/getto-css/preact/design/data"

import { VNodeContent, VNodeKey } from "../../../x_preact/common"

import {
    CheckboxBoardComponent,
    CheckboxBoardContent,
} from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/checkbox"

import { SearchColumnsResource } from "../resource"

export type SearchColumnState = Readonly<{
    key: VNodeKey
    content: VNodeContent
}>

type Props = SearchColumnsResource & SearchColumnsProps
type SearchColumnsProps = CheckboxProps | (CheckboxProps & Readonly<{ title: VNodeContent }>)
type CheckboxProps = ColumnsProps | (ColumnsProps & Readonly<{ block: boolean }>)
type ColumnsProps = Readonly<{ columns: SearchColumnState[] }>
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

    function title() {
        if ("title" in props) {
            return props.title
        }
        return "表示する列"
    }
    function options(): CheckboxBoardContent[] {
        return props.columns.map((column) => ({
            key: column.key,
            value: `${column.key}`,
            label: column.content,
        }))
    }
    function block(): boolean {
        if ("block" in props) {
            return props.block
        }
        return false
    }
}
