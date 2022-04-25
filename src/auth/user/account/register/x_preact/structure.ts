import { h } from "preact"
import { useMemo } from "preact/hooks"
import { html } from "htm/preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { focusClass, listEditLabel } from "../../../../../core/x_preact/design/table"

import { AuthRoleLabels } from "../../input/granted_roles/x_preact/input"
import { ResetTokenDestinationLabel } from "../../../password/reset/token_destination/input/x_preact/destination"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"
import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { ListRegisteredAuthUserAccountAction } from "../action"

import { AuthUserAccount } from "../../kernel/data"

export type ListRegisteredAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccount>

type Summary = {
    // no props
}

export function useRegisteredAuthUserAccountTableStructure(
    list: ListRegisteredAuthUserAccountAction,
): ListRegisteredAuthUserAccountTableStructure {
    return useMemo(() => build(list), [list])
}

function build(
    list: ListRegisteredAuthUserAccountAction,
): ListRegisteredAuthUserAccountTableStructure {
    return tableStructure(rowKey, [
        tableCell("edit", (_key) => ({
            label: "",
            header: linky,
            column: editLink,
        })).alwaysVisible(),

        // TODO builder にしたいかな
        tableCell("login-id", (_key) => ({
            label: "ログインID",
            header: linky,
            column: loginId,
        }))
            .alwaysVisible()
            .border(["leftDouble"]),

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

    function rowKey(row: AuthUserAccount): string {
        return row.loginId
    }

    function loginId(row: AuthUserAccount): VNodeContent {
        return row.loginId
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
