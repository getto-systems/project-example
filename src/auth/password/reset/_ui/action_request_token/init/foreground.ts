import { newRequestResetTokenInfra } from "../../request_token/init"

import { buildRequestResetTokenView } from "./worker/foreground"

import { initRequestResetTokenCoreAction, initRequestResetTokenCoreMaterial } from "../core/impl"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"

import { RequestResetTokenView } from "../resource"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenView(feature: OutsideFeature): RequestResetTokenView {
    return buildRequestResetTokenView(
        initRequestResetTokenCoreAction(
            initRequestResetTokenCoreMaterial(newRequestResetTokenInfra(feature)),
        ),
    )
}
