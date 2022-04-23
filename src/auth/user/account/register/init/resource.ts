import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newRegisterAuthUserAccountConfig } from "./config"

import { initRegisterAuthUserAccountAction, RegisterAuthUserAccountAction } from "../action"

import { newRegisterAuthUserAccountInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newRegisterAuthUserAccountAction(
    feature: OutsideFeature,
): RegisterAuthUserAccountAction {
    return initRegisterAuthUserAccountAction({
        infra: newRegisterAuthUserAccountInfra(feature),
        config: newRegisterAuthUserAccountConfig(),
    })
}
