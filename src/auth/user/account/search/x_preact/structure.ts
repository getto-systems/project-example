import { useMemo } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { sortSign } from "../../../../../z_vendor/getto-css/preact/design/table"
import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"
import { lnir } from "../../../../../z_lib/ui/icon/init/line_icon"

import { SORT_SIGN } from "../../../../../core/x_preact/design/table"
import { iconHtml } from "../../../../../core/x_preact/design/icon"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"

import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { SearchAuthUserAccountAction } from "../action"

import { AuthUserAccountBasket } from "../../kernel/data"
import { SearchAuthUserAccountSortKey } from "../data"

export type SearchAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccountBasket>

type Summary = {
    // no props
}

export function useSearchAuthUserAccountTableStructure(
    search: SearchAuthUserAccountAction,
): SearchAuthUserAccountTableStructure {
    return useMemo(() => {
        const structure = build(search)
        search.columns.setInitialSearchColumns(structure.initialVisibleCells())
        return structure
    }, [search])
}

function build(search: SearchAuthUserAccountAction): SearchAuthUserAccountTableStructure {
    return tableStructure(rowKey, [
        // TODO builder にしたいかな
        tableCell("login-id", (key) => ({
            label: "ログインID",
            header: sort(key),
            column: loginID,
        }))
            .alwaysVisible()
            .border(["rightDouble"]),

        tableCell("granted-roles", (_key) => ({
            label: "権限",
            header: linky,
            column: grantedRoles,
        })),

        tableCell("edit", (_key) => ({
            label: "",
            header: linky,
            column: editLink,
        }))
            .alwaysVisible()
            .border(["left"]),
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

    function rowKey(row: AuthUserAccountBasket): string {
        return row.loginID
    }

    function loginID(row: AuthUserAccountBasket): VNodeContent {
        return row.loginID
    }
    function grantedRoles(row: AuthUserAccountBasket): VNodeContent {
        return row.grantedRoles.join(" / ")
    }

    function editLink(_row: AuthUserAccountBasket): VNodeContent {
        return html`<a href="#">${iconHtml(lnir(["pencil"]))} 編集</a>`
    }
}

interface Decorate {
    (content: VNodeContent): VNodeContent
}
