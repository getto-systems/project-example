import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newChangePasswordConfig, newOverwritePasswordConfig } from "./config"

import {
    ChangePasswordAction,
    initChangePasswordAction,
    initOverwritePasswordAction,
    OverwritePasswordAction,
} from "../action"

import { newChangePasswordInfra, newOverwritePasswordInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordAction(feature: OutsideFeature): ChangePasswordAction {
    return initChangePasswordAction({
        infra: newChangePasswordInfra(feature),
        config: newChangePasswordConfig(),
    })
}

export function newOverwritePasswordAction(feature: OutsideFeature): OverwritePasswordAction {
    return initOverwritePasswordAction({
        infra: newOverwritePasswordInfra(feature),
        config: newOverwritePasswordConfig(),
    })
}
