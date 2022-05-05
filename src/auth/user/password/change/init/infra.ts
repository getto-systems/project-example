import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newChangePasswordRemote } from "./change_remote"
import { newOverwritePasswordRemote } from "./overwrite_remote"

import { ChangePasswordInfra, OverwritePasswordInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordInfra(feature: OutsideFeature): ChangePasswordInfra {
    return {
        changePasswordRemote: newChangePasswordRemote(feature),
    }
}

export function newOverwritePasswordInfra(feature: OutsideFeature): OverwritePasswordInfra {
    return {
        overwritePasswordRemote: newOverwritePasswordRemote(feature),
    }
}
