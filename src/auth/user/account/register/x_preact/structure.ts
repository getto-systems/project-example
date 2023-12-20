import { h } from "preact"

import { PreactContent } from "../../../../../common/x_preact/node"

import { linky } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { ResetTokenDestinationLabel } from "../../../password/reset/token_destination/input/field/x_preact/input"
import { EditLinkForRegisterList } from "../../../../../common/util/list/x_preact/edit_link"

import { TableStructure } from "../../../../../z_vendor/getto-table/preact/core"
import { tableStructure } from "../../../../../z_vendor/getto-table/preact/cell/structure"
import { tableCell } from "../../../../../z_vendor/getto-table/preact/cell/simple"
import { tableClassName } from "../../../../../z_vendor/getto-table/preact/decorator"

import { RegisterAuthUserAccountAction } from "../action"

import { AuthUserAccount, AUTH_USER_ACCOUNT } from "../../kernel/data"
import { authPermissionGranted } from "../../kernel/x_preact/field"

export type RegisteredAuthUserAccountTableStructure = TableStructure<Summary, AuthUserAccount>

type Summary = {
    // no props
}

export function initRegisteredAuthUserAccountTableStructure(
    register: RegisterAuthUserAccountAction,
): RegisteredAuthUserAccountTableStructure {
    return tableStructure(rowKey, [
        tableCell("edit", (_key) => ({
            label: "",
            header: linky,
            column: (data: AuthUserAccount) =>
                h(EditLinkForRegisterList, { focus: register.focus, data }),
        })).alwaysVisible(),

        tableCell("loginId", (key) => ({
            label: AUTH_USER_ACCOUNT[key],
            header: linky,
            column: loginId,
        }))
            .alwaysVisible()
            .border(["leftDouble"]),

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

    function rowKey(row: AuthUserAccount): string {
        return row.loginId
    }

    function loginId(row: AuthUserAccount): PreactContent {
        return row.loginId
    }
    function resetTokenDestination(row: AuthUserAccount): PreactContent {
        return h(ResetTokenDestinationLabel, row)
    }
}
