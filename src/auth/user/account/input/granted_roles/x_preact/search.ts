import { h, VNode } from "preact"

import { VNodeContent } from "../../../../../../z_lib/ui/x_preact/common"

import { search } from "../../../../../../z_vendor/getto-css/preact/design/form"

import {
    CheckboxBoard,
    CheckboxBoardContent,
} from "../../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import { ALL_AUTH_ROLES, authRoleLabel } from "../../../../../../x_content/role"

import { SearchGrantedRolesAction } from "../action"

import { AuthRole } from "../../../../kernel/data"

type Props = Readonly<{
    field: SearchGrantedRolesAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
    }>

export function SearchGrantedRolesField(props: Props): VNode {
    return search({
        title: props.title || "権限",
        help: props.help,
        body: h(CheckboxBoard, {
            input: props.field.grantedRoles,
            options: ALL_AUTH_ROLES.map(roleCheckbox),
        }),
    })

    function roleCheckbox(role: AuthRole): CheckboxBoardContent {
        return {
            key: role,
            value: role,
            label: authRoleLabel(role),
        }
    }
}