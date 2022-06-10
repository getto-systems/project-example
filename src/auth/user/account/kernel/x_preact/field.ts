import { html } from "htm/preact"
import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { authRoleLabel } from "../../../../../x_content/role"

import { CheckboxBoardContent } from "../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"
import { label_gray, label_info } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { AuthRole } from "../../../kernel/data"
import { AuthUserField, TypeAuthUser } from "../data"

type Props<K extends AuthUserField> = Readonly<{ [key in K]: TypeAuthUser<K> }>

export function authUserGrantedRoles({
    grantedRoles,
}: Readonly<{ grantedRoles: readonly AuthRole[] }>): VNodeContent {
    if (grantedRoles.length === 0) {
        return label_gray("権限なし")
    }
    return html`${grantedRoles.map((grantedRole) => {
        return html` ${label_info(authRoleLabel(grantedRole))} `
    })}`
}

export function authUserMemo(data: Props<"memo">): VNodeContent {
    return data.memo
}

export function authRoleCheckboxContent(value: AuthRole): CheckboxBoardContent {
    return {
        key: value,
        value: value,
        label: authRoleLabel(value),
    }
}
