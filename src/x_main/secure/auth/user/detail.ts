import { newModifyAuthUserAccountAction } from "../../../../auth/user/account/modify/detail/resource"
import { newChangeResetTokenDestinationAction } from "../../../../auth/user/password/reset/token_destination/change/detail/resource"
import { newOverwriteLoginIdAction } from "../../../../auth/user/login_id/change/detail/resource"
import { newOverwritePasswordAction } from "../../../../auth/user/password/change/detail/resource"
import { newUnregisterAuthUserAccountAction } from "../../../../auth/user/account/unregister/detail/resource"

import { DetailAuthUserAccountActions } from "../../../../auth/user/account/kernel/x_preact/detail"

import { Atom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../common/util/load/data"
import { LoadableListAtomUpdater } from "../../../../common/util/list/action"

import { AuthUserAccount } from "../../../../auth/user/account/kernel/data"

export function newDetailAuthUserAccountActions(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
): DetailAuthUserAccountActions {
    const modify = newModifyAuthUserAccountAction(data, updater)
    const changeResetTokenDestination = newChangeResetTokenDestinationAction(data, updater)
    const overwriteLoginId = newOverwriteLoginIdAction(data, updater)
    const overwritePassword = newOverwritePasswordAction(data)
    const unregister = newUnregisterAuthUserAccountAction(data, updater)

    return {
        focus: data,
        modify,
        changeResetTokenDestination,
        overwriteLoginId,
        overwritePassword,
        unregister,
    }
}
