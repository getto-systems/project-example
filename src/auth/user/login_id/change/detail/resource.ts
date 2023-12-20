import { newOverwritePasswordConfig } from "./config"
import { newOverwritePasswordInfra } from "./infra"

import { Atom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../common/util/load/data"
import { LoadableListAtomUpdater } from "../../../../../common/util/list/action"
import { initOverwriteLoginIdAction, OverwriteLoginIdAction } from "../action"

import { AuthUserAccount } from "../../../account/kernel/data"

export function newOverwriteLoginIdAction(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
): OverwriteLoginIdAction {
    return initOverwriteLoginIdAction(data, updater, {
        infra: newOverwritePasswordInfra(),
        config: newOverwritePasswordConfig(),
    })
}
