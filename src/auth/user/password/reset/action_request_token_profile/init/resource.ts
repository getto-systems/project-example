import { newRequestResetTokenProfileMaterial } from "./common"

import { initRequestResetTokenProfileAction } from "../init"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { RequestResetTokenProfileResource } from "../resource"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenProfileResource(
    feature: OutsideFeature,
): RequestResetTokenProfileResource {
    return {
        requestToken: initRequestResetTokenProfileAction(
            newRequestResetTokenProfileMaterial(feature),
        ),
    }
}
