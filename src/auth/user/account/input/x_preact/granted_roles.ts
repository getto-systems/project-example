import { html } from "htm/preact"
import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"
import { label_gray, label_info } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import {
    CheckboxBoardComponent,
    CheckboxBoardContent,
} from "../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

import { InputGrantedRolesAction } from "../action"
import {
    EditableBoardAction,
    EditableBoardState,
} from "../../../../../z_vendor/getto-application/board/editable/action"

import { toBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/convert"

import { GrantedAuthRole } from "../data"
import { LoginId } from "../../../login_id/input/data"

type EntryProps = Readonly<{
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly GrantedAuthRole[] }>
    editable: EditableBoardAction
    field: InputGrantedRolesAction
}> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
    }>

export function InputGrantedRolesEntry(props: EntryProps): VNode {
    return h(InputGrantedRolesComponent, {
        ...props,
        editableState: useApplicationAction(props.editable),
    })
}

type Props = EntryProps &
    Readonly<{
        editableState: EditableBoardState
    }>

export function InputGrantedRolesComponent(props: Props): VNode {
    return content()

    function content() {
        const content = {
            title: props.title || "権限",
            help: props.help,
            body: body(),
        }

        return field(content)

        function body(): VNodeContent {
            if (!props.editableState.isEditable) {
                return h(GrantedRoleLabels, { ...props.user })
            }
            return h(CheckboxBoardComponent, {
                input: props.field.grantedRoles,
                options: [grantedRoleCheckbox("user")],
            })
        }

        function grantedRoleCheckbox(grantedRole: GrantedAuthRole): CheckboxBoardContent {
            return {
                key: grantedRole,
                value: toBoardValue(grantedRole),
                label: grantedRoleLabel(grantedRole),
            }
        }
    }
}

export function GrantedRoleLabels({
    grantedRoles,
}: Readonly<{ grantedRoles: readonly GrantedAuthRole[] }>): VNode {
    if (grantedRoles.length === 0) {
        return label_gray("権限なし")
    }
    return html`${grantedRoles.map((grantedRole) => label_info(grantedRoleLabel(grantedRole)))}`
}
export function grantedRoleLabel(grantedRole: GrantedAuthRole): VNodeContent {
    switch (grantedRole) {
        case "user":
            // TODO これはメニューと一緒にしたい
            return "ユーザー管理"
    }
}
