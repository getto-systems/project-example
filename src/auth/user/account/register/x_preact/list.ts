import { PreactNode } from "../../../../../common/x_preact/node"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

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
export function ListRegisteredAuthUserAccount(props: Props): PreactNode {
    const state = useAtom(props.register.list)
    if (!state.isLoad) {
        return emptyRegisteredTable()
    }

    const params = {
        summary: {},
        visibleKeys: props.structure
            .allCells()
            .filter((cell) => cell.isInitiallyVisible)
            .map((cell) => cell.key),
    }

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
