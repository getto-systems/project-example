import { VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    table,
    tableColumn,
    tableHeader,
    tbody,
    thead,
} from "../../../../../z_vendor/getto-css/preact/design/table"
import { emptyTable, takeLongtimeTable } from "../../../../../common/x_preact/design/table"

import { searchColumns } from "../../../../../z_lib/ui/search/columns/x_preact/helper"

import { ListAuthUserAccountAction } from "../action"

import { SearchAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    list: ListAuthUserAccountAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountTable(props: Props): VNode {
    const state = useApplicationAction(props.list)
    const columnsState = useApplicationAction(props.list.columns)

    if (state.type === "try" && state.hasTakenLongtime) {
        return takeLongtimeTable()
    }

    const result = props.list.searchResponse(state)
    const columns = searchColumns(columnsState)
    if (!columns.found || !result.response) {
        return html``
    }
    if (result.response.page.all === 0) {
        return emptyTable()
    }

    const params = { summary: {}, visibleKeys: columns.columns }

    const sticky = props.structure.sticky()
    return table(sticky, [
        thead(tableHeader({ sticky, header: props.structure.header(params) })),
        tbody(
            result.response.users.flatMap((row) =>
                tableColumn({ sticky, column: props.structure.column(params, row) }),
            ),
        ),
    ])
}
