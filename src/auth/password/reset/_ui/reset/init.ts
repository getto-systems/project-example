import { auth_config } from "../../../../_ui/x_outside_feature/config"

import { newResetPasswordRemote } from "./init/remote/reset"

import { newClock } from "../../../../../z_details/_ui/clock/init"
import { toURL } from "../../../../../z_details/_ui/location/init"

import { ResetPasswordDetecter } from "./method"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../../z_details/_ui/location/feature"

import { ResetPasswordInfra } from "./infra"

import { detectResetToken } from "../../../_ui/convert"

export function newResetPasswordLocationDetecter(
    feature: LocationOutsideFeature,
): ResetPasswordDetecter {
    return () => detectResetToken(toURL(feature))
}

export function newResetPasswordInfra(feature: RemoteOutsideFeature): ResetPasswordInfra {
    return {
        reset: newResetPasswordRemote(feature, newClock()),
        config: {
            takeLongtimeThreshold: auth_config.takeLongtimeThreshold,
        },
    }
}
