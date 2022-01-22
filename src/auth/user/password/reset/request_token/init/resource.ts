import { toApplicationView } from "../../../../../../../ui/vendor/getto-application/action/helper"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { newRequestResetTokenConfig } from "./config"
import { newRequestResetTokenInfra } from "./infra"

import { ApplicationView } from "../../../../../../../ui/vendor/getto-application/action/action"
import {
    initRequestResetTokenAction,
    initRequestResetTokenProfileAction,
    RequestResetTokenAction,
    RequestResetTokenProfileAction,
} from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenView(
    feature: OutsideFeature,
): ApplicationView<RequestResetTokenAction> {
    return toApplicationView(
        initRequestResetTokenAction({
            infra: newRequestResetTokenInfra(feature),
            config: newRequestResetTokenConfig(),
        }),
    )
}

export function newRequestResetTokenProfileResource(
    feature: OutsideFeature,
): Readonly<{ requestToken: RequestResetTokenProfileAction }> {
    return {
        requestToken: initRequestResetTokenProfileAction({
            infra: newRequestResetTokenInfra(feature),
            config: newRequestResetTokenConfig(),
        }),
    }
}
