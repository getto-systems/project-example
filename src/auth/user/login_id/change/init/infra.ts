import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newOverrideLoginIdRemote } from "./override_remote"

import {  OverrideLoginIdInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newOverridePasswordInfra(feature: OutsideFeature): OverrideLoginIdInfra {
    return {
        overrideLoginIdRemote: newOverrideLoginIdRemote(feature),
    }
}
