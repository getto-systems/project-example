import { toApplicationView } from "../../../../../../z_vendor/getto-application/action/helper"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { newRequestResetTokenConfig } from "./config"
import { newRequestResetTokenInfra } from "./infra"

import { ApplicationView } from "../../../../../../z_vendor/getto-application/action/action"
import { initRequestResetTokenAction, RequestResetTokenAction } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenView(
    feature: OutsideFeature,
): ApplicationView<RequestResetTokenAction> {
    return toApplicationView(newRequestResetTokenAction(feature))
}

export function newRequestResetTokenAction(feature: OutsideFeature): RequestResetTokenAction {
    return initRequestResetTokenAction({
        infra: newRequestResetTokenInfra(feature),
        config: newRequestResetTokenConfig(),
    })
}
