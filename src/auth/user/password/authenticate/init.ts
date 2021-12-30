import { auth_config } from "../../../x_outside_feature/config"

import { newAuthenticatePasswordRemote } from "./init/remote"
import { newClock } from "../../../../z_lib/ui/clock/init"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { AuthenticatePasswordInfra } from "./infra"

export function newAuthenticatePasswordInfra(
    feature: RemoteOutsideFeature,
): AuthenticatePasswordInfra {
    return {
        authenticateRemote: newAuthenticatePasswordRemote(feature, newClock()),
        config: {
            takeLongtimeThreshold: auth_config.takeLongtimeThreshold,
        },
    }
}
