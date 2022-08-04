import { h } from "preact"
import { useMemo } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { sortSign } from "../../../../../z_vendor/getto-css/preact/design/table"
import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { focusClass, listEditLabel, SORT_SIGN } from "../../../../../common/x_preact/design/table"

import { ResetTokenDestinationLabel } from "../../../password/reset/token_destination/input/x_preact/input"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"
import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { SearchAuthUserAccountAction } from "../action"

import { SearchAuthUserAccountSortKey } from "../data"
import { AuthUserAccount, AUTH_USER_ACCOUNT } from "../../kernel/data"
import { authUserGrantedRoles } from "../../kernel/x_preact/field"

export type SearchAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccount>

type Summary = {
    // no props
}

export function useAuthUserAccountTableStructure(
    search: SearchAuthUserAccountAction,
): SearchAuthUserAccountTableStructure {
    return useMemo(() => build(search), [search])
}

function build(search: SearchAuthUserAccountAction): SearchAuthUserAccountTableStructure {
    return tableStructure(rowKey, [
        tableCell("edit", (_key) => ({
            label: "",
            header: linky,
            column: editLink,
        })).alwaysVisible(),

        tableCell("loginId", (key) => ({
            label: AUTH_USER_ACCOUNT[key],
            header: sort(key),
            column: loginId,
        }))
            .alwaysVisible()
            .border(["leftDouble"]),

        tableCell("memo", (key) => ({
            label: AUTH_USER_ACCOUNT[key],
            header: linky,
            column: memo,
        })).border(["left"]),

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

    function sort(key: SearchAuthUserAccountSortKey): Decorate {
        return (content) => html`<a href="#" onClick=${onClick}>${content} ${sign()}</a>`

        function sign() {
            const currentSort = search.currentSort()
            return sortSign(SORT_SIGN, currentSort, key)
        }
        function onClick(e: Event) {
            e.preventDefault()
            search.sort(key)
        }
    }

    function rowKey(row: AuthUserAccount): string {
        return row.loginId
    }

    function loginId(row: AuthUserAccount): VNodeContent {
        return row.loginId
    }
    function memo(row: AuthUserAccount): VNodeContent {
        return row.memo
    }
    function resetTokenDestination(row: AuthUserAccount): VNodeContent {
        return h(ResetTokenDestinationLabel, row)
    }

    function editLink(row: AuthUserAccount): VNodeContent {
        const isFocused = search.list.focus.isFocused(row)
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
            search.list.focus.change(row)
        }
    }
}

interface Decorate {
    (content: VNodeContent): VNodeContent
}
