import { newChangePasswordMaterial } from "./common"

import { initChangePasswordAction } from "../init"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import { LocationOutsideFeature } from "../../../../../z_lib/ui/location/feature"

import { ChangePasswordResource } from "../resource"

export function newChangePasswordResource(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): ChangePasswordResource {
    return {
        change: initChangePasswordAction(newChangePasswordMaterial(feature)),
    }
}
