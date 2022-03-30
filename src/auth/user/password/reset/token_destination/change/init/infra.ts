import { RemoteOutsideFeature } from "../../../../../../../z_lib/ui/remote/feature"

import { newModifyAuthUserAccountRemote } from "./change_remote"

import { ChangeResetTokenDestinationInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newChangeResetTokenDestinationInfra(
    feature: OutsideFeature,
): ChangeResetTokenDestinationInfra {
    return {
        changeDestinationRemote: newModifyAuthUserAccountRemote(feature),
    }
}
