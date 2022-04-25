import { h } from "preact"
import { useMemo } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { sortSign } from "../../../../../z_vendor/getto-css/preact/design/table"
import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { focusClass, listEditLabel, SORT_SIGN } from "../../../../../core/x_preact/design/table"

import { AuthRoleLabels } from "../../input/granted_roles/x_preact/input"
import { ResetTokenDestinationLabel } from "../../../password/reset/token_destination/input/x_preact/destination"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"
import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { ListAuthUserAccountAction } from "../action"

import { SearchAuthUserAccountSortKey } from "../data"
import { AuthUserAccount } from "../../kernel/data"

export type SearchAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccount>

type Summary = {
    // no props
}

export function useAuthUserAccountTableStructure(
    list: ListAuthUserAccountAction,
): SearchAuthUserAccountTableStructure {
    return useMemo(() => {
        const structure = build(list)
        list.columns.set(structure.initialVisibleCells())
        return structure
    }, [list])
}

function build(list: ListAuthUserAccountAction): SearchAuthUserAccountTableStructure {
    return tableStructure(rowKey, [
        tableCell("edit", (_key) => ({
            label: "",
            header: linky,
            column: editLink,
        })).alwaysVisible(),

        // TODO builder にしたいかな
        tableCell("login-id", (key) => ({
            // TODO このラベルをどこかに統一したいが...
            label: "ログインID",
            header: sort(key),
            column: loginId,
        }))
            .alwaysVisible()
            .border(["leftDouble"]),

        tableCell("memo", (_key) => ({
            label: "備考",
            header: linky,
            column: memo,
        })).border(["left"]),

        tableCell("granted-roles", (_key) => ({
            label: "権限",
            header: linky,
            column: grantedRoles,
        })).border(["left"]),

        tableCell("reset-token-destination", (_key) => ({
            label: "パスワードリセット用Eメール",
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
            const currentSort = list.currentSort()
            return sortSign(SORT_SIGN, currentSort, key)
        }
        function onClick(e: Event) {
            e.preventDefault()
            list.sort(key)
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
    function grantedRoles(row: AuthUserAccount): VNodeContent {
        return h(AuthRoleLabels, row)
    }
    function resetTokenDestination(row: AuthUserAccount): VNodeContent {
        return h(ResetTokenDestinationLabel, row)
    }

    function editLink(row: AuthUserAccount): VNodeContent {
        const isFocused = list.focused.isFocused(row)
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
            list.focused.focus(row)
        }
    }
}

interface Decorate {
    (content: VNodeContent): VNodeContent
}
