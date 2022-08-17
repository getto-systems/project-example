import { h, VNode } from "preact"
import { VNodeContent } from "../../../../../../z_vendor/getto-css/preact/common"

import { label, search_double } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { checkboxOptions } from "../../../../../../common/x_preact/design/checkbox"

import { CheckboxBoard } from "../../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"
import { authRoleCheckboxContent } from "../../../kernel/x_preact/field"

import { AuthUserGrantedRolesFilterAction } from "../action"

import { ALL_AUTH_ROLES } from "../../../../../../x_content/role"

import { AUTH_USER_ACCOUNT } from "../../../kernel/data"
import { prepared } from "../../../../../../z_lib/ui/prepare/data"

type Props = Readonly<{
    field: AuthUserGrantedRolesFilterAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
    }>

export function AuthUserGrantedRolesFilter(props: Props): VNode {
    return search_double({
        label: label,
        title: props.title || AUTH_USER_ACCOUNT["grantedRoles"],
        help: props.help,
        body: h(CheckboxBoard, {
            input: props.field.input,
            options: checkboxOptions(prepared(ALL_AUTH_ROLES), authRoleCheckboxContent),
        }),
    })
}
