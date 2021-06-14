import { newResetPasswordRemote } from "./infra/remote/reset"

import { newClock } from "../../../../../z_details/_ui/clock/init"
import { toURL } from "../../../../../z_details/_ui/location/init"

import { ResetPasswordDetecter } from "./method"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../../z_details/_ui/location/feature"

import { delaySecond } from "../../../../../z_details/_ui/config/infra"
import { ResetPasswordInfra } from "./infra"

import { detectResetToken } from "../converter"

export function newResetPasswordLocationDetecter(
    feature: LocationOutsideFeature,
): ResetPasswordDetecter {
    return () => detectResetToken(toURL(feature))
}

export function newResetPasswordInfra(feature: RemoteOutsideFeature): ResetPasswordInfra {
    return {
        reset: newResetPasswordRemote(feature, newClock()),
        config: {
            takeLongtimeThreshold: delaySecond(1),
        },
    }
}
