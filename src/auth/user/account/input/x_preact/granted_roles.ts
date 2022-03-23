import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"

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

import { GrantedRole } from "../data"
import { AuthUserAccountBasket } from "../../kernel/data"
import { label_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

type EntryProps = Readonly<{
    user: AuthUserAccountBasket
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
                if (props.user.grantedRoles.length === 0) {
                    return label_gray("権限なし")
                }
                return props.user.grantedRoles.join(" / ")
            }
            return h(CheckboxBoardComponent, {
                input: props.field.grantedRoles,
                options: [grantedRoleCheckbox("user")],
            })
        }

        function grantedRoleCheckbox(grantedRole: GrantedRole): CheckboxBoardContent {
            return {
                key: grantedRole,
                value: toBoardValue(grantedRole),
                label: label(grantedRole),
            }
        }
        function label(grantedRole: GrantedRole): VNodeContent {
            switch (grantedRole) {
                case "user":
                    // TODO これはメニューと一緒にしたい
                    return "ユーザー管理"
            }
        }
    }
}
