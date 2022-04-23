import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newUnregisterAuthUserAccountRemote } from "./unregister_remote"

import { UnregisterAuthUserAccountInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newUnregisterAuthUserAccountInfra(
    feature: OutsideFeature,
): UnregisterAuthUserAccountInfra {
    return {
        unregisterUserRemote: newUnregisterAuthUserAccountRemote(feature),
    }
}
