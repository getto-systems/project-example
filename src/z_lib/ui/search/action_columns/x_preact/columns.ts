import { h, VNode } from "preact"

import { field } from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { tableViewColumns } from "../../../../../../ui/vendor/getto-css/preact/design/data"

import { VNodeContent } from "../../../../../example/x_preact/design/common"

import {
    CheckboxBoardComponent,
    CheckboxBoardContent,
} from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/checkbox"

import { SearchColumnsResource } from "../resource"

type SearchProps = SearchOptions | (Readonly<{ title: VNodeContent }> & SearchOptions)
type SearchOptions =
    | Readonly<{ label: { (key: string): VNodeContent } }>
    | Readonly<{ label: { (key: string): VNodeContent }; block: boolean }>

type Props = SearchColumnsResource & SearchProps
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
        return props.field.full.map((key) => ({
            key,
            value: key,
            label: props.label(key),
        }))
    }
    function block(): boolean {
        if ("block" in props) {
            return props.block
        }
        return false
    }
}
