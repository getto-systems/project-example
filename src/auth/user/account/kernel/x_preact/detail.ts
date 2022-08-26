import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { ModifyAuthUserAccount } from "../../modify/x_preact/modify"
import { OverwriteLoginId } from "../../../login_id/change/x_preact/overwrite_login_id"
import { OverwritePassword } from "../../../password/change/x_preact/overwrite_password"
import { ChangeResetTokenDestination } from "../../../password/reset/token_destination/change/x_preact/change"
import { UnregisterAuthUserAccount } from "../../unregister/x_preact/unregister"

import { OverwriteLoginIdAction } from "../../../login_id/change/action"
import { OverwritePasswordAction } from "../../../password/change/action"
import { ModifyAuthUserAccountAction } from "../../modify/action"
import { ChangeResetTokenDestinationAction } from "../../../password/reset/token_destination/change/action"
import { UnregisterAuthUserAccountAction } from "../../unregister/action"
import { ApplicationState } from "../../../../../z_vendor/getto-application/action/action"
import { FocusState } from "../../../../../z_lib/ui/list/action"

import { AuthUserAccount } from "../data"

export type DetailAuthUserAccountActions = Readonly<{
    focus: ApplicationState<FocusState<AuthUserAccount>>
    modify: ModifyAuthUserAccountAction
    changeResetTokenDestination: ChangeResetTokenDestinationAction
    overwriteLoginId: OverwriteLoginIdAction
    overwritePassword: OverwritePasswordAction
    unregister: UnregisterAuthUserAccountAction
}>
export function DetailAuthUserAccount(props: DetailAuthUserAccountActions): VNode {
    return html`${[
        container([
            h(ModifyAuthUserAccount, props),
            h(ChangeResetTokenDestination, {
                focus: props.focus,
                change: props.changeResetTokenDestination,
            }),
        ]),
        container([
            h(OverwriteLoginId, { overwrite: props.overwriteLoginId }),
            h(OverwritePassword, { overwrite: props.overwritePassword }),
        ]),
        container([h(UnregisterAuthUserAccount, { unregister: props.unregister })]),
    ]}`
}
