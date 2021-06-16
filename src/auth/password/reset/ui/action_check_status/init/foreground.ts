import { newCheckSendingStatusMaterial } from "./common"

import { initCheckResetTokenSendingStatusView } from "../impl"
import { initCheckResetTokenSendingStatusCoreAction } from "../core/impl"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../../../z_details/_ui/location/feature"

import { CheckResetTokenSendingStatusView } from "../resource"

export function newCheckPasswordResetSendingStatus(
    feature: RemoteOutsideFeature & LocationOutsideFeature,
): CheckResetTokenSendingStatusView {
    return initCheckResetTokenSendingStatusView(
        initCheckResetTokenSendingStatusCoreAction(newCheckSendingStatusMaterial(feature)),
    )
}
