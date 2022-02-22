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
import { EMPTY_TABLE } from "../../../../../core/x_preact/design/table"

import { SearchAuthUserAccountAction, SearchAuthUserAccountState } from "../action"
import { SearchColumnsState } from "../../../../../z_lib/ui/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

import { SearchColumns } from "../../../../../z_lib/ui/search/columns/data"
import { SearchAuthUserAccountRemoteResponse } from "../data"

type EntryProps = Readonly<{
    search: SearchAuthUserAccountAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountTableEntry(resource: EntryProps): VNode {
    return h(SearchAuthUserAccountTableComponent, {
        ...resource,
        state: useApplicationAction(resource.search),
        columns: useApplicationAction(resource.search.columns),
    })
}

type Props = EntryProps &
    Readonly<{
        state: SearchAuthUserAccountState
        columns: SearchColumnsState
    }>
export function SearchAuthUserAccountTableComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, columns }: Props): VNode {
        switch (columns.type) {
            case "repository-error":
            case "initial-search":
                return EMPTY_CONTENT

            case "succeed-to-load":
            case "succeed-to-save":
                if (state.type === "succeed-to-search") {
                    return content({ columns: columns.columns, response: state.response })
                }
                return EMPTY_CONTENT
        }
    }

    type Content = Readonly<{
        columns: SearchColumns
        response: SearchAuthUserAccountRemoteResponse
    }>

    function content({ columns, response }: Content): VNode {
        if (response.page.all === 0) {
            return EMPTY_TABLE
        }

        const params = { summary: {}, visibleKeys: columns }

        const sticky = props.structure.sticky()
        const header = props.structure.header(params)

        return table(sticky, [
            thead(tableHeader({ sticky, header })),
            tbody(
                response.users.flatMap((row) =>
                    tableColumn({ sticky, column: props.structure.column(params, row) }),
                ),
            ),
        ])
    }
}

const EMPTY_CONTENT = html``
