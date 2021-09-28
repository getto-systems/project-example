import { auth_config } from "../../../x_outside_feature/config"

import { newAuthenticatePasswordRemote } from "./init/remote/authenticate"
import { newClock } from "../../../../z_lib/ui/clock/init"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { AuthenticatePasswordInfra } from "./infra"

export function newAuthenticatePasswordInfra(
    feature: RemoteOutsideFeature,
): AuthenticatePasswordInfra {
    return {
        authenticate: newAuthenticatePasswordRemote(feature, newClock()),
        config: {
            takeLongtimeThreshold: auth_config.takeLongtimeThreshold,
        },
    }
}
