import { toApplicationView } from "../../../../../../ui/vendor/getto-application/action/helper"

import { newRequestResetTokenInfra } from "../../request_token/init"

import { initRequestResetTokenAction, initRequestResetTokenMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { RequestResetTokenView } from "../resource"
import { RequestResetTokenMaterial } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenView(feature: OutsideFeature): RequestResetTokenView {
    return initRequestResetTokenView(
        initRequestResetTokenMaterial(newRequestResetTokenInfra(feature)),
    )
}

export function initRequestResetTokenView(
    material: RequestResetTokenMaterial,
): RequestResetTokenView {
    return toApplicationView(initRequestResetTokenAction(material))
}
