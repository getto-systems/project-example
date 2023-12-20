import { newModifyAuthUserAccountConfig } from "./config"
import { newModifyAuthUserAccountInfra } from "./infra"

import { Atom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../common/util/load/data"
import { LoadableListAtomUpdater } from "../../../../../common/util/list/action"
import { initModifyAuthUserAccountAction, ModifyAuthUserAccountAction } from "../action"

import { AuthUserAccount } from "../../kernel/data"

export function newModifyAuthUserAccountAction(
    data: Atom<LoadState<AuthUserAccount>>,
    updater: LoadableListAtomUpdater<AuthUserAccount>,
): ModifyAuthUserAccountAction {
    return initModifyAuthUserAccountAction(data, updater, {
        infra: newModifyAuthUserAccountInfra(),
        config: newModifyAuthUserAccountConfig(),
    })
}
