import { html } from "htm/preact"
import { h, VNode } from "preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { useEditableState } from "../../../../../../z_vendor/getto-application/board/editable/x_preact/hooks"

import { inputField, label } from "../../../../../../z_vendor/getto-css/preact/design/form"
import {
    label_gray,
    label_info,
} from "../../../../../../z_vendor/getto-css/preact/design/highlight"

import {
    CheckboxBoard,
    CheckboxBoardContent,
} from "../../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import { ALL_AUTH_ROLES, authRoleLabel } from "../../../../../../x_content/role"

import { InputGrantedAuthRolesAction } from "../action"
import { EditableBoardAction } from "../../../../../../z_vendor/getto-application/board/editable/action"

import { AuthRole } from "../../../../kernel/data"

type Props = Readonly<{ field: InputGrantedAuthRolesAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
        edit: Readonly<{
            data: Readonly<{ grantedRoles: readonly AuthRole[] }>
            editable: EditableBoardAction
        }>
    }>

export function GrantedRolesField(props: Props): VNode {
    const editableState = useEditableState(props.edit)

    return inputField({
        title: props.title || "権限",
        help: props.help,
        label,
        state: { type: "normal" },
        body: editableState.isEditable
            ? h(CheckboxBoard, {
                  input: props.field.input,
                  options: ALL_AUTH_ROLES.map(roleCheckbox),
              })
            : h(AuthRoleLabels, { ...editableState.data }),
    })

    function roleCheckbox(role: AuthRole): CheckboxBoardContent {
        return {
            key: role,
            value: role,
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