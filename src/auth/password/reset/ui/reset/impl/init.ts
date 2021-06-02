import { newResetPasswordRemote } from "../infra/remote/reset"
import { newClock } from "../../../../../../../ui/vendor/getto-application/infra/clock/init"
import { newLocationDetecter } from "../../../../../../../ui/vendor/getto-application/location/init"

import { detectResetToken } from "./core"

import { delaySecond } from "../../../../../../../ui/vendor/getto-application/infra/config/infra"
import { ResetPasswordInfra } from "../infra"

import { ResetPasswordLocationDetecter } from "../method"
import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { LocationOutsideFeature } from "../../../../../../../ui/vendor/getto-application/location/infra"

export function newResetPasswordLocationDetecter(
    feature: LocationOutsideFeature,
): ResetPasswordLocationDetecter {
    return newLocationDetecter(feature, detectResetToken)
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
