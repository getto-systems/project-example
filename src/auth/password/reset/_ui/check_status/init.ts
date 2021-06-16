import { newSendResetTokenRemote } from "./infra/remote/send_token"
import { newGetResetTokenSendingStatusRemote } from "./infra/remote/get_sending_status"

import { toURL } from "../../../../../z_details/_ui/location/init"

import { CheckResetTokenSendingStatusDetecter } from "./method"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../../z_details/_ui/location/feature"

import { limit, waitSecond } from "../../../../../z_details/_ui/config/infra"
import { CheckResetTokenSendingStatusInfra } from "./infra"

import { detectResetSessionID } from "../convert"

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
