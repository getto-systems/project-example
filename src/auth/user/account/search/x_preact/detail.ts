import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { ModifyAuthUserAccount } from "../../modify/x_preact/modify"
import { OverrideLoginId } from "../../../login_id/change/x_preact/override_login_id"
import { OverridePassword } from "../../../password/change/x_preact/override_password"
import { ChangeResetTokenDestination } from "../../../password/reset/token_destination/change/x_preact/change"
import { BackToListButton } from "../../../../../core/x_preact/button/back_to_list_button"

import { DetailAuthUserAccountAction } from "../action"
import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverrideLoginIdAction } from "../../../login_id/change/action"
import { OverridePasswordAction } from "../../../password/change/action"
import { ModifyAuthUserAccountAction } from "../../modify/action"
import { ChangeResetTokenDestinationAction } from "../../../password/reset/token_destination/change/action"

import { AuthUserAccount } from "../../kernel/data"

type Props = Readonly<{
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
export function DetailAuthUserAccount(props: Props): VNode {
    return html`${[container([box_grow({ body: backToListButton() })]), ...content()]}`

    function content(): VNode[] {
        if (!props.user.found) {
            return [
                container([
                    box_grow({ body: notice_gray(["指定されたユーザーが見つかりませんでした"]) }),
                ]),
            ]
        }

        const user = props.user.user

        return [
            container([
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
                    onSuccess: (resetTokenDestination) => {
                        props.detail.update({ ...user, resetTokenDestination })
                    },
                }),
            ]),
            container([
                h(OverrideLoginId, {
                    ...props.overrideLoginId,
                    user,
                    onSuccess: (loginId) => {
                        props.detail.update({ ...user, loginId })
                    },
                }),
                h(OverridePassword, { ...props.overridePassword, user }),
            ]),
        ]
    }

    function backToListButton(): VNode {
        return h(BackToListButton, { onClick })

        function onClick() {
            props.detail.close()
        }
    }
}
