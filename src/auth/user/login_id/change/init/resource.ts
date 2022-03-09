import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import {  newOverridePasswordConfig } from "./config"

import {
    initOverrideLoginIdAction,
    OverrideLoginIdAction,
} from "../action"

import {  newOverridePasswordInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newOverrideLoginIdAction(feature: OutsideFeature): OverrideLoginIdAction {
    return initOverrideLoginIdAction({
        infra: newOverridePasswordInfra(feature),
        config: newOverridePasswordConfig(),
    })
}
