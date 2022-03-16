import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"
import { button_cancel } from "../../../../../z_vendor/getto-css/preact/design/form"

import { BACK_TO_LIST_BUTTON } from "../../../../../core/x_preact/design/table"

import { ModifyAuthUserAccountEntry } from "../../modify/x_preact/modify"
import { OverrideLoginIdEntry } from "../../../login_id/change/x_preact/override_login_id"
import { OverridePasswordEntry } from "../../../password/change/x_preact/override_password"

import { DetailAuthUserAccountAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverrideLoginIdAction } from "../../../login_id/change/action"
import { OverridePasswordAction } from "../../../password/change/action"
import { ModifyAuthUserAccountAction } from "../../modify/action"

import { AuthUserAccountBasket } from "../../kernel/data"

type EntryProps = Readonly<{
    detail: DetailAuthUserAccountAction
    modify: Readonly<{
        editable: EditableBoardAction
        modify: ModifyAuthUserAccountAction
    }>
    overrideLoginId: Readonly<{
        editable: EditableBoardAction
        override: OverrideLoginIdAction
    }>
    overridePassword: Readonly<{
        editable: EditableBoardAction
        override: OverridePasswordAction
    }>
    user: Readonly<{ found: false }> | Readonly<{ found: true; user: AuthUserAccountBasket }>
}>
export function DetailAuthUserAccountEntry(props: EntryProps): VNode {
    return html`${[container([h(CloseButtonComponent, props)]), content()]}`

    function content(): VNode {
        if (!props.user.found) {
            return container([
                box_grow({ body: notice_gray(["指定されたユーザーが見つかりませんでした"]) }),
            ])
        }

        const user = props.user.user

        return container([
            h(ModifyAuthUserAccountEntry, { ...props.modify, user }),
            h(OverrideLoginIdEntry, { ...props.overrideLoginId, user }),
            h(OverridePasswordEntry, { ...props.overridePassword, user }),
        ])
    }
}

type CloseButtonProps = EntryProps
function CloseButtonComponent(props: CloseButtonProps): VNode {
    return box_grow({ body: button_cancel({ label: BACK_TO_LIST_BUTTON, onClick }) })

    function onClick() {
        props.detail.close()
    }
}
