import { RemoteOutsideFeature } from "../../../../../../../z_lib/ui/remote/feature"

import { newChangeResetTokenDestinationConfig } from "./config"
import { newChangeResetTokenDestinationInfra } from "./infra"

import {
    initChangeResetTokenDestinationAction,
    ChangeResetTokenDestinationAction,
    ChangeResetTokenDestinationEntry,
} from "../action"
import { ModifyFieldHandler } from "../../../../../../../z_lib/ui/modify/action"

type OutsideFeature = RemoteOutsideFeature
export function newChangeResetTokenDestinationAction(feature: OutsideFeature): Readonly<{
    action: ChangeResetTokenDestinationAction
    handler: ModifyFieldHandler<ChangeResetTokenDestinationEntry>
}> {
    return initChangeResetTokenDestinationAction({
        infra: newChangeResetTokenDestinationInfra(feature),
        config: newChangeResetTokenDestinationConfig(),
    })
}
