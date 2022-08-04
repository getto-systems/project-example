import { VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    table,
    tableColumn,
    tableHeader,
    tbody,
    thead,
} from "../../../../../z_vendor/getto-css/preact/design/table"
import { emptyRegisteredTable } from "../../../../../common/x_preact/design/table"

import { ListRegisteredAction } from "../../../../../z_lib/ui/list/action"

import { ListRegisteredAuthUserAccountTableStructure } from "./structure"

import { AuthUserAccount } from "../../kernel/data"

type Props = Readonly<{
    list: ListRegisteredAction<AuthUserAccount>
    structure: ListRegisteredAuthUserAccountTableStructure
}>
export function ListRegisteredAuthUserAccountTable(props: Props): VNode {
    const state = useApplicationAction(props.list)

    if (!state.isLoad) {
        return emptyRegisteredTable()
    }

    const params = { summary: {}, visibleKeys: props.structure.initialVisibleCells() }

    const sticky = props.structure.sticky()
    return table(sticky, [
        thead(tableHeader({ sticky, header: props.structure.header(params) })),
        tbody(
            state.data.flatMap((row) =>
                tableColumn({ sticky, column: props.structure.column(params, row) }),
            ),
        ),
    ])
}
