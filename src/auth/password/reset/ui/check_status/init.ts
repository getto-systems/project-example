import { newSendResetTokenRemote } from "./infra/remote/send_token"
import { newGetResetTokenSendingStatusRemote } from "./infra/remote/get_sending_status"

import { toURL } from "../../../../../../ui/vendor/getto-application/location/init"

import { CheckResetTokenSendingStatusDetecter } from "./method"

import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/feature"
import { LocationOutsideFeature } from "../../../../../../ui/vendor/getto-application/location/feature"

import { limit, waitSecond } from "../../../../../../ui/vendor/getto-application/infra/config/infra"
import { CheckResetTokenSendingStatusInfra } from "./infra"

import { detectResetSessionID } from "../converter"

export function newCheckResetTokenSendingStatusLocationDetecter(
    feature: LocationOutsideFeature,
): CheckResetTokenSendingStatusDetecter {
    return () => detectResetSessionID(toURL(feature))
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
