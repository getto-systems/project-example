import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { newRequestResetTokenConfig } from "./config"
import { newRequestResetTokenInfra } from "./infra"

import { initRequestResetTokenAction, RequestResetTokenAction } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenAction(feature: OutsideFeature): RequestResetTokenAction {
    return initRequestResetTokenAction({
        infra: newRequestResetTokenInfra(feature),
        config: newRequestResetTokenConfig(),
    })
}
