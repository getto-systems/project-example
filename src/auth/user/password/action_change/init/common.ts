import { newChangePasswordInfra } from "../../change/init"

import { initChangePasswordMaterial } from "../init"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { ChangePasswordMaterial } from "../action"

export function newChangePasswordMaterial(feature: RemoteOutsideFeature): ChangePasswordMaterial {
    return initChangePasswordMaterial(newChangePasswordInfra(feature))
}
