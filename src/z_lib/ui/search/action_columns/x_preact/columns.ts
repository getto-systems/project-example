import { h, VNode } from "preact"

import { field } from "../../../../../../ui/vendor/getto-css/preact/design/form"
import { tableViewColumns } from "../../../../../../ui/vendor/getto-css/preact/design/data"

import { VNodeContent } from "../../../../../example/x_preact/design/common"

import {
    CheckboxBoardComponent,
    CheckboxBoardContent,
} from "../../../../../../ui/vendor/getto-application/board/action_input/x_preact/checkbox"

import { SearchColumnsResource } from "../resource"
import { useLayoutEffect } from "preact/hooks"

type SearchProps = CheckboxProps | (Readonly<{ title: VNodeContent }> & CheckboxProps)
type CheckboxProps = ColumnProps | (ColumnProps & Readonly<{ block: boolean }>)
type ColumnProps = Readonly<{ columns: SearchColumnState[] }>

export type SearchColumnState = Readonly<{ key: string; content: VNodeContent; isVisible: boolean }>

type Props = SearchColumnsResource & SearchProps
export function SearchColumnsComponent(props: Props): VNode {
    useLayoutEffect(() => {
        props.field.load(
            props.columns.filter((column) => column.isVisible).map((column) => `${column.key}`),
        )
    }, [props.field, props.columns])

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
