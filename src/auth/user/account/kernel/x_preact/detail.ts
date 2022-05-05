import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { ModifyAuthUserAccount } from "../../modify/x_preact/modify"
import { OverwriteLoginId } from "../../../login_id/change/x_preact/overwrite_login_id"
import { OverwritePassword } from "../../../password/change/x_preact/overwrite_password"
import { ChangeResetTokenDestination } from "../../../password/reset/token_destination/change/x_preact/change"
import { UnregisterAuthUserAccount } from "../../unregister/x_preact/unregister"

import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverwriteLoginIdAction } from "../../../login_id/change/action"
import { OverwritePasswordAction } from "../../../password/change/action"
import { ModifyAuthUserAccountAction } from "../../modify/action"
import { ChangeResetTokenDestinationAction } from "../../../password/reset/token_destination/change/action"
import { UnregisterAuthUserAccountAction } from "../../unregister/action"

import { AuthUserAccount } from "../../kernel/data"
import { LoginId } from "../../../login_id/kernel/data"

export type DetailAuthUserAccountActions = Readonly<{
    modify: Readonly<{
        editable: EditableBoardAction
        modify: ModifyAuthUserAccountAction
    }>
    changeResetTokenDestination: Readonly<{
        editable: EditableBoardAction
        change: ChangeResetTokenDestinationAction
    }>
    overwriteLoginId: Readonly<{
        editable: EditableBoardAction
        overwrite: OverwriteLoginIdAction
    }>
    overwritePassword: Readonly<{
        editable: EditableBoardAction
        overwrite: OverwritePasswordAction
    }>
    unregister: Readonly<{
        editable: EditableBoardAction
        unregister: UnregisterAuthUserAccountAction
    }>
}>
type Props = DetailAuthUserAccountActions &
    Readonly<{
        user: AuthUserAccount
        onModify: { (loginId: LoginId, user: AuthUserAccount): void }
        onUnregister: { (loginId: LoginId): void }
    }>
export function DetailAuthUserAccount(props: Props): VNode {
    const user = props.user

    return html`${[
        container([
            h(ModifyAuthUserAccount, {
                ...props.modify,
                user,
                onSuccess: (fields) => {
                    props.onModify(user.loginId, { ...user, ...fields })
                },
            }),
            h(ChangeResetTokenDestination, {
                ...props.changeResetTokenDestination,
                user,
                onSuccess: (resetTokenDestination) => {
                    props.onModify(user.loginId, { ...user, resetTokenDestination })
                },
            }),
        ]),
        container([
            h(OverwriteLoginId, {
                ...props.overwriteLoginId,
                user,
                onSuccess: (loginId) => {
                    props.onModify(user.loginId, { ...user, loginId })
                },
            }),
            h(OverwritePassword, { ...props.overwritePassword, user }),
        ]),
        container([
            h(UnregisterAuthUserAccount, {
                ...props.unregister,
                user,
                onSuccess: (loginId) => {
                    props.onUnregister(loginId)
                },
            }),
        ]),
    ]}`
}
