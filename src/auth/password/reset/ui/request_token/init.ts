import { newRequestResetTokenRemote } from "./infra/remote/request_token"

import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/feature"

import { delaySecond } from "../../../../../../ui/vendor/getto-application/infra/config/infra"
import { RequestResetTokenInfra } from "./infra"

export function newRequestResetTokenInfra(feature: RemoteOutsideFeature): RequestResetTokenInfra {
    return {
        requestToken: newRequestResetTokenRemote(feature),
        config: {
            takeLongtimeThreshold: delaySecond(1),
        },
    }
}
