import { toApplicationView } from "../../../../../../../ui/vendor/getto-application/action/helper"

import { newRequestResetTokenInfra } from "../../request_token/init"

import { initRequestResetTokenAction, initRequestResetTokenMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"

import { RequestResetTokenView } from "../resource"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenView(feature: OutsideFeature): RequestResetTokenView {
    return toApplicationView(
        initRequestResetTokenAction(
            initRequestResetTokenMaterial(newRequestResetTokenInfra(feature)),
        ),
    )
}
