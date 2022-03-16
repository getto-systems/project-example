import { h, VNode } from "preact"

import { field } from "../../../../../z_vendor/getto-css/preact/design/form"

import { VNodeContent } from "../../../../../z_lib/ui/x_preact/common"

import { InputGrantedRolesAction } from "../action"

import { GrantedRole } from "../data"
import { toBoardValue } from "../../../../../z_vendor/getto-application/board/kernel/convert"
import {
    CheckboxBoardComponent,
    CheckboxBoardContent,
} from "../../../../../z_vendor/getto-application/board/input/x_preact/checkbox"

type Props = Readonly<{ field: InputGrantedRolesAction }> &
    Partial<{
        title: VNodeContent
        help: readonly VNodeContent[]
    }>

export function InputGrantedRolesComponent(props: Props): VNode {
    return content()

    function content() {
        const content = {
            title: title(),
            body: h(CheckboxBoardComponent, {
                input: props.field.grantedRoles,
                options: [destinationCheckbox("user")],
            }),
            help: help(),
        }

        return field(content)

        function destinationCheckbox(grantedRole: GrantedRole): CheckboxBoardContent {
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
    function title(): VNodeContent {
        if (props.title) {
            return props.title
        }
        // TODO 一覧のカラムと一緒にしたい
        return "権限"
    }
    function help(): readonly VNodeContent[] {
        if (props.help) {
            return props.help
        }
        return []
    }
}
