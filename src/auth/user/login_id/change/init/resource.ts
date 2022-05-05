import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import {  newOverwritePasswordConfig } from "./config"

import {
    initOverwriteLoginIdAction,
    OverwriteLoginIdAction,
} from "../action"

import {  newOverwritePasswordInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature
export function newOverwriteLoginIdAction(feature: OutsideFeature): OverwriteLoginIdAction {
    return initOverwriteLoginIdAction({
        infra: newOverwritePasswordInfra(feature),
        config: newOverwritePasswordConfig(),
    })
}
