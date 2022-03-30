import { RemoteOutsideFeature } from "../../../../../../../z_lib/ui/remote/feature"

import { newChangeResetTokenDestinationConfig } from "./config"
import { newChangeResetTokenDestinationInfra } from "./infra"

import { initChangeResetTokenDestinationAction, ChangeResetTokenDestinationAction } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newChangeResetTokenDestinationAction(
    feature: OutsideFeature,
): ChangeResetTokenDestinationAction {
    return initChangeResetTokenDestinationAction({
        infra: newChangeResetTokenDestinationInfra(feature),
        config: newChangeResetTokenDestinationConfig(),
    })
}
