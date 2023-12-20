import { newUnregisterAuthUserAccountConfig } from "./config"
import { newUnregisterAuthUserAccountInfra } from "./infra"

import { Atom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../common/util/load/data"
import { LoadableListAtomUpdater } from "../../../../../common/util/list/action"
import { initUnregisterAuthUserAccountAction, UnregisterAuthUserAccountAction } from "../action"

import { AuthUserAccount } from "../../kernel/data"

export function newUnregisterAuthUserAccountAction(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
): UnregisterAuthUserAccountAction {
    return initUnregisterAuthUserAccountAction(data, updater, {
        infra: newUnregisterAuthUserAccountInfra(),
        config: newUnregisterAuthUserAccountConfig(),
    })
}
