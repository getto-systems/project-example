import { useMemo } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../example/x_preact/design/common"

import { sortSign } from "../../../../../../ui/vendor/getto-css/preact/design/data"
import { linky } from "../../../../../../ui/vendor/getto-css/preact/design/highlight"

import { siteSortSign } from "../../../../../example/x_preact/design/table"
import { icon } from "../../../../../example/x_preact/design/icon"

import { TableStructure } from "../../../../../../ui/vendor/getto-table/preact/core"

import { tableStructure } from "../../../../../../ui/vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../../ui/vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../../ui/vendor/getto-table/preact/decorator"

import { SearchUserAccountAction } from "../action"

import { UserAccount } from "../../kernel/data"

export type SearchUserAccountTableStructure = TableStructure<Summary, UserAccount>

type Summary = {
    // no props
}

export function useSearchUserAccountTableStructure(
    search: SearchUserAccountAction,
): SearchUserAccountTableStructure {
    return useMemo(() => build(search), [search])
}

function build(search: SearchUserAccountAction): SearchUserAccountTableStructure {
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

    function rowKey(row: UserAccount): string {
        return row.loginID
    }

    function loginID(row: UserAccount): VNodeContent {
        return row.loginID
    }
    function grantedRoles(row: UserAccount): VNodeContent {
        return row.grantedRoles.join(" / ")
    }

    function editLink(_row: UserAccount): VNodeContent {
        return html`<a href="#">${icon("pencil")} 編集</a>`
    }
}

interface Decorate {
    (content: VNodeContent): VNodeContent
}
