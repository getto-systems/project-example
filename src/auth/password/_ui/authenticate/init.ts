import { newAuthenticatePasswordRemote } from "./infra/remote/authenticate"
import { newClock } from "../../../../z_details/_ui/clock/init"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"

import { delaySecond } from "../../../../z_details/_ui/config/infra"
import { AuthenticatePasswordInfra } from "./infra"

export function newAuthenticatePasswordInfra(
    feature: RemoteOutsideFeature,
): AuthenticatePasswordInfra {
    return {
        authenticate: newAuthenticatePasswordRemote(feature),
        clock: newClock(),
        config: {
            takeLongtimeThreshold: delaySecond(1),
        },
    }
}
