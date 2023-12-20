import { html } from "htm/preact"
import { PreactNode } from "../../../../../common/x_preact/node"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import {
    table,
    tableColumn,
    tableHeader,
    tbody,
    thead,
} from "../../../../../z_vendor/getto-css/preact/design/table"
import { emptyTable, takeLongtimeTable } from "../../../../../common/x_preact/design/table"

import { SearchAuthUserAccountAction } from "../action"
import { SearchColumnsBoard } from "../../../../../common/util/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
    columns: SearchColumnsBoard
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountTable(props: Props): PreactNode {
    const connectState = useAtom(props.search.connect)
    const listState = useAtom(props.search.list)
    const visibleKeys = useAtom(props.columns.value)

    if (connectState.isConnecting && connectState.hasTakenLongtime) {
        return takeLongtimeTable()
    }

    if (!listState.isLoad) {
        return html``
    }
    if (listState.data.length === 0) {
        return emptyTable()
    }

    const params = {
        summary: {},
        visibleKeys,
    }

    const sticky = props.structure.sticky()
    return table(sticky, [
        thead(tableHeader({ sticky, header: props.structure.header(params) })),
        tbody(
            listState.data.flatMap((row) =>
                tableColumn({ sticky, column: props.structure.column(params, row) }),
            ),
        ),
    ])
}
