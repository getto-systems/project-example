import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    table,
    tableColumn,
    tableHeader,
    tbody,
    thead,
} from "../../../../../z_vendor/getto-css/preact/design/table"
import {
    EMPTY_TABLE,
    TAKE_LONGTIME_TO_SEARCH_TABLE,
} from "../../../../../core/x_preact/design/table"

import { searchResponse } from "../../../../../z_lib/ui/search/kernel/x_preact/helper"
import { searchColumns } from "../../../../../z_lib/ui/search/columns/x_preact/helper"

import { ListAuthUserAccountAction, SearchAuthUserAccountState } from "../action"
import { SearchColumnsState } from "../../../../../z_lib/ui/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

import { SearchColumns } from "../../../../../z_lib/ui/search/columns/data"
import { SearchAuthUserAccountRemoteResponse } from "../data"

type EntryProps = Readonly<{
    list: ListAuthUserAccountAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountTableEntry(resource: EntryProps): VNode {
    return h(SearchAuthUserAccountTableComponent, {
        ...resource,
        state: useApplicationAction(resource.list),
        columnsState: useApplicationAction(resource.list.columns),
    })
}

type Props = EntryProps &
    Readonly<{
        state: SearchAuthUserAccountState
        columnsState: SearchColumnsState
    }>
export function SearchAuthUserAccountTableComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, columnsState }: Props): VNode {
        if (state.type === "take-longtime-to-search") {
            return TAKE_LONGTIME_TO_SEARCH_TABLE
        } else {
            const response = searchResponse(state)
            const columns = searchColumns(columnsState)
            if (!columns.found || !response.found) {
                return EMPTY_CONTENT
            }
            if (response.response.page.all === 0) {
                return EMPTY_TABLE
            }
            return content({ columns: columns.columns, response: response.response })
        }
    }

    type Content = Readonly<{
        columns: SearchColumns
        response: SearchAuthUserAccountRemoteResponse
    }>

    function content({ columns, response }: Content): VNode {
        const params = { summary: {}, visibleKeys: columns }

        const sticky = props.structure.sticky()
        return table(sticky, [
            thead(tableHeader({ sticky, header: props.structure.header(params) })),
            tbody(
                response.users.flatMap((row) =>
                    tableColumn({ sticky, column: props.structure.column(params, row) }),
                ),
            ),
        ])
    }
}

const EMPTY_CONTENT = html``
