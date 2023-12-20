import { html } from "htm/preact"
import { PreactContent } from "../../../../../common/x_preact/node"

import { authPermissionLabel } from "../../../../../x_content/permission"

import { CheckboxBoardContent } from "../../../../../common/util/board/input/x_preact/checkbox"
import { label_gray, label_info } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { AuthPermission } from "../../../kernel/data"
import { AuthUserField, TypeAuthUser } from "../data"

type Props<K extends AuthUserField> = Readonly<{ [key in K]: TypeAuthUser<K> }>

export function authPermissionGranted({
    granted,
}: Readonly<{ granted: readonly AuthPermission[] }>): PreactContent {
    if (granted.length === 0) {
        return label_gray("権限なし")
    }
    return html`${granted.map((permission) => {
        return html` ${label_info(authPermissionLabel(permission))} `
    })}`
}

export function authUserMemo(data: Props<"memo">): PreactContent {
    return data.memo
}

export function authPermissionCheckboxContent(value: AuthPermission): CheckboxBoardContent {
    return {
        key: value,
        value: value,
        label: authPermissionLabel(value),
    }
}
