import {
    newCheckResetTokenSendingStatusInfra,
    newCheckResetTokenSendingStatusLocationDetecter,
} from "../../check_status/init"

import {
    initCheckResetTokenSendingStatusCoreMaterial,
    initCheckResetTokenSendingStatusCoreMaterialPod,
} from "../core/impl"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../../../z_details/_ui/location/feature"

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
