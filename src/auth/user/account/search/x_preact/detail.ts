import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"
import { button_cancel } from "../../../../../z_vendor/getto-css/preact/design/form"

import { BACK_TO_LIST_BUTTON } from "../../../../../core/x_preact/design/table"

import { ModifyAuthUserAccount } from "../../modify/x_preact/modify"
import { OverrideLoginId } from "../../../login_id/change/x_preact/override_login_id"
import { OverridePassword } from "../../../password/change/x_preact/override_password"
import { ChangeResetTokenDestination } from "../../../password/reset/token_destination/change/x_preact/change"

import { DetailAuthUserAccountAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverrideLoginIdAction } from "../../../login_id/change/action"
import { OverridePasswordAction } from "../../../password/change/action"
import { ModifyAuthUserAccountAction } from "../../modify/action"
import { ChangeResetTokenDestinationAction } from "../../../password/reset/token_destination/change/action"

import { AuthUserAccount } from "../../kernel/data"

type EntryProps = Readonly<{
    detail: DetailAuthUserAccountAction
    modify: Readonly<{
        editable: EditableBoardAction
        modify: ModifyAuthUserAccountAction
    }>
    changeResetTokenDestination: Readonly<{
        editable: EditableBoardAction
        change: ChangeResetTokenDestinationAction
    }>
    overrideLoginId: Readonly<{
        editable: EditableBoardAction
        override: OverrideLoginIdAction
    }>
    overridePassword: Readonly<{
        editable: EditableBoardAction
        override: OverridePasswordAction
    }>
    user: Readonly<{ found: false }> | Readonly<{ found: true; user: AuthUserAccount }>
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
            h(ModifyAuthUserAccount, {
                ...props.modify,
                user,
                onSuccess: (fields) => {
                    props.detail.update({ ...user, ...fields })
                },
            }),
            h(ChangeResetTokenDestination, {
                ...props.changeResetTokenDestination,
                user,
                onSuccess: (destination) => {
                    props.detail.update({ ...user, resetTokenDestination: destination })
                },
            }),
            h(OverrideLoginId, { ...props.overrideLoginId, user }),
            h(OverridePassword, { ...props.overridePassword, user }),
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
