import { newRequestResetTokenInfra } from "../../request_token/init"

import { initRequestResetTokenProfileMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { RequestResetTokenProfileMaterial } from "../action"

export function newRequestResetTokenProfileMaterial(
    feature: RemoteOutsideFeature,
): RequestResetTokenProfileMaterial {
    return initRequestResetTokenProfileMaterial(newRequestResetTokenInfra(feature))
}
