import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { VNodeContent, VNodeKey } from "../../../x_preact/common"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"
import { tableViewColumns } from "../../../../../z_vendor/getto-css/preact/design/table"
import {
    CheckboxBoardComponent,
    CheckboxBoardContent,
} from "../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import { toBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/convert"

import { SearchColumnsAction, SearchColumnsState } from "../action"

export type SearchColumnContent = Readonly<{
    key: VNodeKey
    content: VNodeContent
}>

type EntryProps = Readonly<{
    field: SearchColumnsAction
    columns: readonly SearchColumnContent[]
}> &
    Partial<{
        title: VNodeContent
        block: boolean
    }>
export function SearchColumnsEntry(props: EntryProps): VNode {
    return h(SearchColumnsComponent, {
        ...props,
        state: useApplicationAction(props.field),
    })
}

type Props = EntryProps &
    Readonly<{
        state: SearchColumnsState
    }>
export function SearchColumnsComponent(props: Props): VNode {
    return field({
        title: title(),
        body: [tableViewColumns(checkbox())],
    })

    function title(): VNodeContent {
        if (props.title) {
            return props.title
        }
        return "表示する列"
    }
    function checkbox(): VNode {
        return h(CheckboxBoardComponent, {
            input: props.field.input,
            options: options(),
            block: block(),
        })

        function options(): readonly CheckboxBoardContent[] {
            return props.columns.map((column) => ({
                key: column.key,
                value: toBoardValue(`${column.key}`),
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
}
