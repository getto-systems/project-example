import { html } from "htm/preact"
import { VNode } from "preact"

import {
    label_gray,
    label_info,
} from "../../../../../../z_vendor/getto-css/preact/design/highlight"

import { authRoleLabel } from "../../../../../../x_content/role"

import { AuthRole } from "../../../../kernel/data"

export function AuthRoleLabels({
    grantedRoles,
}: Readonly<{ grantedRoles: readonly AuthRole[] }>): VNode {
    if (grantedRoles.length === 0) {
        return label_gray("権限なし")
    }
    return html`${grantedRoles.map((grantedRole) => {
        return html` ${label_info(authRoleLabel(grantedRole))} `
    })}`
}
