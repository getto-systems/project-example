import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { ModifyAuthUserAccount } from "../../modify/x_preact/modify"
import { OverrideLoginId } from "../../../login_id/change/x_preact/override_login_id"
import { OverridePassword } from "../../../password/change/x_preact/override_password"
import { ChangeResetTokenDestination } from "../../../password/reset/token_destination/change/x_preact/change"
import { UnregisterAuthUserAccount } from "../../unregister/x_preact/unregister"

import { EditableBoardAction } from "../../../../../z_vendor/getto-application/board/editable/action"
import { OverrideLoginIdAction } from "../../../login_id/change/action"
import { OverridePasswordAction } from "../../../password/change/action"
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
    overrideLoginId: Readonly<{
        editable: EditableBoardAction
        override: OverrideLoginIdAction
    }>
    overridePassword: Readonly<{
        editable: EditableBoardAction
        override: OverridePasswordAction
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
            h(OverrideLoginId, {
                ...props.overrideLoginId,
                user,
                onSuccess: (loginId) => {
                    props.onModify(user.loginId, { ...user, loginId })
                },
            }),
            h(OverridePassword, { ...props.overridePassword, user }),
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
