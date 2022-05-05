import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newOverwriteLoginIdRemote } from "./overwrite_remote"

import {  OverwriteLoginIdInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newOverwritePasswordInfra(feature: OutsideFeature): OverwriteLoginIdInfra {
    return {
        overwriteLoginIdRemote: newOverwriteLoginIdRemote(feature),
    }
}
