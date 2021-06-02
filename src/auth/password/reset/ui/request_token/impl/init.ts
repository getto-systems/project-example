import { newRequestResetTokenRemote } from "../infra/remote/request_token"

import { delaySecond } from "../../../../../../../ui/vendor/getto-application/infra/config/infra"
import { RequestResetTokenInfra } from "../infra"
import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/infra"

export function newRequestResetTokenInfra(feature: RemoteOutsideFeature): RequestResetTokenInfra {
    return {
        requestToken: newRequestResetTokenRemote(feature),
        config: {
            takeLongtimeThreshold: delaySecond(1),
        },
    }
}
