import { h } from "preact"
import { useMemo } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { focusClass, listEditLabel } from "../../../../../common/x_preact/design/table"

import { ResetTokenDestinationLabel } from "../../../password/reset/token_destination/input/x_preact/input"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"
import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { ListRegisteredAction } from "../../../../../z_lib/ui/list/action"

import { AuthUserAccount, AUTH_USER_ACCOUNT } from "../../kernel/data"
import { authUserGrantedRoles } from "../../kernel/x_preact/field"

export type ListRegisteredAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccount>

type Summary = {
    // no props
}

export function useRegisteredAuthUserAccountTableStructure(
    list: ListRegisteredAction<AuthUserAccount>,
): ListRegisteredAuthUserAccountTableStructure {
    return useMemo(() => build(list), [list])
}

function build(
    list: ListRegisteredAction<AuthUserAccount>,
): ListRegisteredAuthUserAccountTableStructure {
    return tableStructure(rowKey, [
        tableCell("edit", (_key) => ({
            label: "",
            header: linky,
            column: editLink,
        })).alwaysVisible(),

        tableCell("loginId", (key) => ({
            label: AUTH_USER_ACCOUNT[key],
            header: linky,
            column: loginId,
        }))
            .alwaysVisible()
            .border(["leftDouble"]),

        tableCell("grantedRoles", (key) => ({
            label: AUTH_USER_ACCOUNT[key],
            header: linky,
            column: (row: AuthUserAccount) => authUserGrantedRoles(row),
        })).border(["left"]),

        tableCell("resetTokenDestination", (key) => ({
            label: AUTH_USER_ACCOUNT[key],
            header: linky,
            column: resetTokenDestination,
        })).border(["left"]),
    ])
        .decorateRow(tableClassName(["row_hover"]))
        .stickyHeader()
        .freeze()

    function rowKey(row: AuthUserAccount): string {
        return row.loginId
    }

    function loginId(row: AuthUserAccount): VNodeContent {
        return row.loginId
    }
    function resetTokenDestination(row: AuthUserAccount): VNodeContent {
        return h(ResetTokenDestinationLabel, row)
    }

    function editLink(row: AuthUserAccount): VNodeContent {
        const isFocused = list.focus.isFocused(row)
        return html`<a
            href="#"
            id="${isFocused ? "focused" : undefined}"
            class="${focusClass(isFocused)}"
            onClick=${onClick}
        >
            ${listEditLabel(isFocused)}
        </a>`

        function onClick(e: Event) {
            e.preventDefault()
            list.focus.change(row)
        }
    }
}
