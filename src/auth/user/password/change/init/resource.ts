import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newChangePasswordConfig, newOverridePasswordConfig } from "./config"

import {
    ChangePasswordAction,
    initChangePasswordAction,
    initOverridePasswordAction,
    OverridePasswordAction,
} from "../action"

import { newChangePasswordInfra, newOverridePasswordInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordAction(feature: OutsideFeature): ChangePasswordAction {
    return initChangePasswordAction({
        infra: newChangePasswordInfra(feature),
        config: newChangePasswordConfig(),
    })
}

export function newOverridePasswordAction(feature: OutsideFeature): OverridePasswordAction {
    return initOverridePasswordAction({
        infra: newOverridePasswordInfra(feature),
        config: newOverridePasswordConfig(),
    })
}
