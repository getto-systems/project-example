import { newSendResetTokenRemote } from "./infra/remote/send_token"
import { newGetResetTokenSendingStatusRemote } from "./infra/remote/get_sending_status"

import { newLocationDetecter } from "../../../../../../ui/vendor/getto-application/location/init"

import { CheckResetTokenSendingStatusDetecter } from "./method"

import { limit, waitSecond } from "../../../../../../ui/vendor/getto-application/infra/config/infra"
import { CheckResetTokenSendingStatusInfra } from "./infra"
import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { LocationOutsideFeature } from "../../../../../../ui/vendor/getto-application/location/infra"

import { detectResetSessionID } from "../converter"

export function newCheckResetTokenSendingStatusLocationDetecter(
    feature: LocationOutsideFeature,
): CheckResetTokenSendingStatusDetecter {
    return newLocationDetecter(feature, detectResetSessionID)
}

export function newCheckResetTokenSendingStatusInfra(
    feature: RemoteOutsideFeature,
): CheckResetTokenSendingStatusInfra {
    return {
        sendToken: newSendResetTokenRemote(feature),
        getStatus: newGetResetTokenSendingStatusRemote(feature),
        config: {
            wait: waitSecond(0.25),
            limit: limit(40),
        },
    }
}
