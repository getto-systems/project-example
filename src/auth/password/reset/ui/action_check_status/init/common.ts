import {
    newCheckResetTokenSendingStatusInfra,
    newCheckResetTokenSendingStatusLocationDetecter,
} from "../../check_status/init"

import {
    initCheckResetTokenSendingStatusCoreMaterial,
    initCheckResetTokenSendingStatusCoreMaterialPod,
} from "../core/impl"

import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/feature"
import { LocationOutsideFeature } from "../../../../../../../ui/vendor/getto-application/location/infra"

import {
    CheckResetTokenSendingStatusCoreMaterial,
    CheckResetTokenSendingStatusCoreMaterialPod,
} from "../core/action"

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
