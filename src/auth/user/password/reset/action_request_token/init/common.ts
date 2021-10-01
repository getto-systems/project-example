import { newRequestResetTokenInfra } from "../../request_token/init"

import { initRequestResetTokenMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { RequestResetTokenMaterial } from "../action"

export function newRequestResetTokenMaterial(
    feature: RemoteOutsideFeature,
): RequestResetTokenMaterial {
    return initRequestResetTokenMaterial(newRequestResetTokenInfra(feature))
}