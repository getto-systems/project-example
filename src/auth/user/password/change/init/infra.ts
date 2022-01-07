import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newChangePasswordRemote } from "./change_remote"

import { ChangePasswordInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordInfra(feature: OutsideFeature): ChangePasswordInfra {
    return {
        changePasswordRemote: newChangePasswordRemote(feature),
    }
}
