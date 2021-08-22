import { newLogoutInfra } from "../../logout/init"

import { initLogoutCoreAction, initLogoutCoreMaterial } from "../init"

import { RepositoryOutsideFeature } from "../../../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { LogoutResource } from "../resource"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutResource(feature: OutsideFeature): LogoutResource {
    return {
        logout: initLogoutCoreAction(initLogoutCoreMaterial(newLogoutInfra(feature))),
    }
}
