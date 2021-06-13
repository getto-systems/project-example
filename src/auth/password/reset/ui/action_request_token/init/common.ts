import { newRequestResetTokenInfra } from "../../request_token/init"

import { initRequestResetTokenCoreMaterial } from "../core/impl"

import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/feature"

import { RequestResetTokenCoreMaterial } from "../core/action"

export function newRequestResetTokenCoreMaterial(
    feature: RemoteOutsideFeature,
): RequestResetTokenCoreMaterial {
    return initRequestResetTokenCoreMaterial(newRequestResetTokenInfra(feature))
}
