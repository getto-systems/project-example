import { newLogoutInfra } from "../../logout/init"

import { initLogoutAction, initLogoutMaterial } from "../init"

import { RepositoryOutsideFeature } from "../../../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { LogoutResource } from "../resource"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutResource(feature: OutsideFeature): LogoutResource {
    return {
        logout: initLogoutAction(initLogoutMaterial(newLogoutInfra(feature))),
    }
}
