import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newUnregisterAuthUserAccountConfig } from "./config"

import { initUnregisterAuthUserAccountAction, UnregisterAuthUserAccountAction } from "../action"

import { newUnregisterAuthUserAccountInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newUnregisterAuthUserAccountAction(
    feature: OutsideFeature,
): UnregisterAuthUserAccountAction {
    return initUnregisterAuthUserAccountAction({
        infra: newUnregisterAuthUserAccountInfra(feature),
        config: newUnregisterAuthUserAccountConfig(),
    })
}
