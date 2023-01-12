import { h, VNode } from "preact"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../common/x_preact/vnode"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { sortSign } from "../../../../../z_vendor/getto-css/preact/design/table"
import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { focusClass, listEditLabel, SORT_SIGN } from "../../../../../common/x_preact/design/table"

import { ResetTokenDestinationLabel } from "../../../password/reset/token_destination/input/x_preact/input"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"
import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { SearchAuthUserAccountAction } from "../action"
import { focusedData } from "../../../../../common/util/list/action"

import { SearchAuthUserAccountSortKey } from "../data"
import { AuthUserAccount, AUTH_USER_ACCOUNT } from "../../kernel/data"
import { authPermissionGranted } from "../../kernel/x_preact/field"
import { ScrollPosition } from "../../../../../common/util/scroll/data"

export type SearchAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccount>

type Summary = {
    // no props
}

export function initSearchAuthUserAccountTableStructure(
    search: SearchAuthUserAccountAction,
): SearchAuthUserAccountTableStructure {
    return tableStructure(rowKey, [
        tableCell("edit", (_key) => ({
            label: "",
            header: linky,
            column: (data: AuthUserAccount) => h(EditLink, { data }),
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

        tableCell("granted", (key) => ({
            label: AUTH_USER_ACCOUNT[key],
            header: linky,
            column: (row: AuthUserAccount) => authPermissionGranted(row),
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

    function EditLink(props: Readonly<{ data: AuthUserAccount }>): VNode {
        const focusState = useApplicationState(search.list.focus.state)

        const data = focusedData(focusState)
        const isFocused = data.isFocused && data.data === props.data

        return html`<a
            href="#"
            id="${isFocused ? "focused" : undefined}"
            class="${focusClass(isFocused)}"
            onClick=${onClick}
        >
            ${listEditLabel()}
        </a>`

        function onClick(e: Event) {
            e.preventDefault()
            if (e.target instanceof HTMLElement) {
                e.target.blur()
            }

            search.list.focus.change(props.data, currentScrollPosition())

            function currentScrollPosition(): ScrollPosition {
                return {
                    y:
                        focusState.type === "close"
                            ? document.documentElement.scrollTop
                            : document.getElementById("sidebar")?.scrollTop || 0,
                }
            }
        }
    }
}

interface Decorate {
    (content: VNodeContent): VNodeContent
}
