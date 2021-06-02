import { newAuthenticatePasswordRemote } from "../infra/remote/authenticate"
import { newClock } from "../../../../../../ui/vendor/getto-application/infra/clock/init"

import { delaySecond } from "../../../../../../ui/vendor/getto-application/infra/config/infra"

import { AuthenticatePasswordInfra } from "../infra"
import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/infra"

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
