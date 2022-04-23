import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newModifyAuthUserAccountConfig } from "./config"

import { initModifyAuthUserAccountAction, ModifyAuthUserAccountAction } from "../action"

import { newModifyAuthUserAccountInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newModifyAuthUserAccountAction(
    feature: OutsideFeature,
): ModifyAuthUserAccountAction {
    return initModifyAuthUserAccountAction({
        infra: newModifyAuthUserAccountInfra(feature),
        config: newModifyAuthUserAccountConfig(),
    })
}
