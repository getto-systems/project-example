import { newCheckSendingStatusMaterial } from "./common"

import { initCheckResetTokenSendingStatusView } from "../impl"
import { initCheckResetTokenSendingStatusCoreAction } from "../core/impl"

import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { LocationOutsideFeature } from "../../../../../../../ui/vendor/getto-application/location/infra"

import { CheckResetTokenSendingStatusView } from "../resource"

export function newCheckPasswordResetSendingStatus(
    feature: RemoteOutsideFeature & LocationOutsideFeature,
): CheckResetTokenSendingStatusView {
    return initCheckResetTokenSendingStatusView(
        initCheckResetTokenSendingStatusCoreAction(newCheckSendingStatusMaterial(feature)),
    )
}
