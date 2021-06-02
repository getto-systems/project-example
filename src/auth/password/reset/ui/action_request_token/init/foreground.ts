import { newRequestResetTokenInfra } from "../../request_token/impl/init"

import { buildRequestResetTokenView } from "./worker/foreground"

import { initRequestResetTokenCoreAction, initRequestResetTokenCoreMaterial } from "../core/impl"

import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/infra"

import { RequestResetTokenView } from "../resource"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenView(feature: OutsideFeature): RequestResetTokenView {
    return buildRequestResetTokenView(
        initRequestResetTokenCoreAction(
            initRequestResetTokenCoreMaterial(newRequestResetTokenInfra(feature)),
        ),
    )
}
