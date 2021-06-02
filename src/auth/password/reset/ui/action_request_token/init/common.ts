import { newRequestResetTokenInfra } from "../../request_token/impl/init"

import { initRequestResetTokenCoreMaterial } from "../core/impl"

import { RequestResetTokenCoreMaterial } from "../core/action"
import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/infra"

export function newRequestResetTokenCoreMaterial(
    feature: RemoteOutsideFeature,
): RequestResetTokenCoreMaterial {
    return initRequestResetTokenCoreMaterial(newRequestResetTokenInfra(feature))
}
