import { useMemo } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { sortSign } from "../../../../../z_vendor/getto-css/preact/design/data"
import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { siteSortSign } from "../../../../../core/x_preact/design/table"
import { icon } from "../../../../../core/x_preact/design/icon"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"

import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { SearchAuthUserAccountAction } from "../action"

import { AuthUserAccountBasket } from "../../kernel/data"

export type SearchAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccountBasket>

type Summary = {
    // no props
}

export function useSearchAuthUserAccountTableStructure(
    search: SearchAuthUserAccountAction,
): SearchAuthUserAccountTableStructure {
    return useMemo(() => build(search), [search])
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

        tableCell("granted-roles", (key) => ({
            label: "権限",
            header: sort(key),
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

    function sort(key: string): Decorate {
        return (content) => html`<a href="#" onClick=${onClick}>${content} ${sign()}</a>`

        function sign() {
            const currentSort = search.currentSort()
            return sortSign(siteSortSign, currentSort, key)
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
        return html`<a href="#">${icon("pencil")} 編集</a>`
    }
}

interface Decorate {
    (content: VNodeContent): VNodeContent
}
