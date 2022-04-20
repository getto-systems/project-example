import { h, VNode } from "preact"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"

import {
    CheckboxBoard,
    CheckboxBoardContent,
} from "../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import { authRoleLabel } from "../../../../../x_content/role"

import { SearchGrantedRolesAction } from "../action"

import { AuthRole } from "../../../kernel/data"

type Props = Readonly<{
    field: SearchGrantedRolesAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
    }>

export function SearchGrantedRoles(props: Props): VNode {
    return field({
        title: props.title || "権限",
        help: props.help,
        body: body(),
    })

    function body(): VNodeContent {
        return h(CheckboxBoard, {
            input: props.field.grantedRoles,
            options: [roleCheckbox("user")],
        })
    }

    function roleCheckbox(role: AuthRole): CheckboxBoardContent {
        return {
            key: role,
            value: role,
            label: authRoleLabel(role),
        }
    }
}
