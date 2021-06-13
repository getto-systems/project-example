import { newRequestResetTokenRemote } from "./infra/remote/request_token"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { delaySecond } from "../../../../../z_details/_ui/config/infra"
import { RequestResetTokenInfra } from "./infra"

export function newRequestResetTokenInfra(feature: RemoteOutsideFeature): RequestResetTokenInfra {
    return {
        requestToken: newRequestResetTokenRemote(feature),
        config: {
            takeLongtimeThreshold: delaySecond(1),
        },
    }
}
