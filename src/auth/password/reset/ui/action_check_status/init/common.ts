import {
    newCheckResetTokenSendingStatusInfra,
    newCheckResetTokenSendingStatusLocationDetecter,
} from "../../check_status/init"

import {
    initCheckResetTokenSendingStatusCoreMaterial,
    initCheckResetTokenSendingStatusCoreMaterialPod,
} from "../core/impl"

import {
    CheckResetTokenSendingStatusCoreMaterial,
    CheckResetTokenSendingStatusCoreMaterialPod,
} from "../core/action"
import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { LocationOutsideFeature } from "../../../../../../../ui/vendor/getto-application/location/infra"

export function newCheckSendingStatusMaterial(
    feature: RemoteOutsideFeature & LocationOutsideFeature,
): CheckResetTokenSendingStatusCoreMaterial {
    return initCheckResetTokenSendingStatusCoreMaterial(
        newCheckResetTokenSendingStatusInfra(feature),
        newCheckResetTokenSendingStatusLocationDetecter(feature),
    )
}
export function newCheckSendingStatusMaterialPod(
    feature: RemoteOutsideFeature,
): CheckResetTokenSendingStatusCoreMaterialPod {
    return initCheckResetTokenSendingStatusCoreMaterialPod(
        newCheckResetTokenSendingStatusInfra(feature),
    )
}
