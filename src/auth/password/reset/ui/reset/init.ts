import { newResetPasswordRemote } from "./infra/remote/reset"

import { newClock } from "../../../../../../ui/vendor/getto-application/infra/clock/init"
import { newDetecter } from "../../../../../../ui/vendor/getto-application/location/init"

import { ResetPasswordDetecter } from "./method"

import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/feature"
import { LocationOutsideFeature } from "../../../../../../ui/vendor/getto-application/location/infra"

import { delaySecond } from "../../../../../../ui/vendor/getto-application/infra/config/infra"
import { ResetPasswordInfra } from "./infra"

import { detectResetToken } from "../converter"

export function newResetPasswordLocationDetecter(
    feature: LocationOutsideFeature,
): ResetPasswordDetecter {
    return newDetecter(feature, detectResetToken)
}

export function newResetPasswordInfra(feature: RemoteOutsideFeature): ResetPasswordInfra {
    return {
        reset: newResetPasswordRemote(feature),
        clock: newClock(),
        config: {
            takeLongtimeThreshold: delaySecond(1),
        },
    }
}
