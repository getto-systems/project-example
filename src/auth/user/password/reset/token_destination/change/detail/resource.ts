import { newChangeResetTokenDestinationConfig } from "./config"
import { newChangeResetTokenDestinationInfra } from "./infra"

import { Atom } from "../../../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../../../common/util/load/data"
import { LoadableListAtomUpdater } from "../../../../../../../common/util/list/action"
import { initChangeResetTokenDestinationAction, ChangeResetTokenDestinationAction } from "../action"

import { AuthUserAccount } from "../../../../../account/kernel/data"

export function newChangeResetTokenDestinationAction(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
): ChangeResetTokenDestinationAction {
    return initChangeResetTokenDestinationAction(data, updater, {
        infra: newChangeResetTokenDestinationInfra(),
        config: newChangeResetTokenDestinationConfig(),
    })
}
