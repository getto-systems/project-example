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

import { SearchAuthUserAccountAction } from "../action"

import { SearchAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountTable(props: Props): VNode {
    const state = useApplicationAction(props.search)
    const listState = useApplicationAction(props.search.list)
    const _columnsState = useApplicationAction(props.search.columns)

    if (state.type === "try" && state.hasTakenLongtime) {
        return takeLongtimeTable()
    }

    if (!listState.isLoad || listState.data.type === "failed") {
        return html``
    }
    if (listState.data.response.page.all === 0) {
        return emptyTable()
    }

    const params = { summary: {}, visibleKeys: props.search.columns.get() }

    const sticky = props.structure.sticky()
    return table(sticky, [
        thead(tableHeader({ sticky, header: props.structure.header(params) })),
        tbody(
            listState.data.response.list.flatMap((row) =>
                tableColumn({ sticky, column: props.structure.column(params, row) }),
            ),
        ),
    ])
}
