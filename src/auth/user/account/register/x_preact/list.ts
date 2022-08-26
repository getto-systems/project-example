import { VNode } from "preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    table,
    tableColumn,
    tableHeader,
    tbody,
    thead,
} from "../../../../../z_vendor/getto-css/preact/design/table"

import { emptyRegisteredTable } from "../../../../../common/x_preact/design/table"

import { RegisterAuthUserAccountAction } from "../action"

import { RegisteredAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    register: RegisterAuthUserAccountAction
    structure: RegisteredAuthUserAccountTableStructure
}>
export function ListRegisteredAuthUserAccount(props: Props): VNode {
    const state = useApplicationState(props.register.list.state)
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
