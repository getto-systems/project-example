import { html } from "htm/preact"
import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"
import { label_gray, label_info } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import {
    CheckboxBoard,
    CheckboxBoardContent,
} from "../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import { InputGrantedRolesAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"

import { toBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/convert"

import { AuthRole } from "../../../kernel/data"
import { LoginId } from "../../../login_id/kernel/data"
import { authRoleLabel } from "../../../../../x_content/role"

type Props = Readonly<{
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>
    editable: EditableBoardAction
    field: InputGrantedRolesAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
    }>

export function InputAuthRoles(props: Props): VNode {
    const editableState = useApplicationAction(props.editable)

    return field({
        title: props.title || "権限",
        help: props.help,
        body: body(),
    })

    function body(): VNodeContent {
        if (!editableState.isEditable) {
            return h(AuthRoleLabels, { ...props.user })
        }
        return h(CheckboxBoard, {
            input: props.field.grantedRoles,
            options: [roleCheckbox("user")],
        })
    }

    function roleCheckbox(role: AuthRole): CheckboxBoardContent {
        return {
            key: role,
            value: toBoardValue(role),
            label: authRoleLabel(role),
        }
    }
}

export function AuthRoleLabels({
    grantedRoles,
}: Readonly<{ grantedRoles: readonly AuthRole[] }>): VNode {
    if (grantedRoles.length === 0) {
        return label_gray("権限なし")
    }
    return html`${grantedRoles.map((grantedRole) => label_info(authRoleLabel(grantedRole)))}`
}
