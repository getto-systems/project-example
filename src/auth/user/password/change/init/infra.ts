import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newChangePasswordRemote } from "./change_remote"
import { newOverridePasswordRemote } from "./override_remote"

import { ChangePasswordInfra, OverridePasswordInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordInfra(feature: OutsideFeature): ChangePasswordInfra {
    return {
        changePasswordRemote: newChangePasswordRemote(feature),
    }
}

export function newOverridePasswordInfra(feature: OutsideFeature): OverridePasswordInfra {
    return {
        overridePasswordRemote: newOverridePasswordRemote(feature),
    }
}
