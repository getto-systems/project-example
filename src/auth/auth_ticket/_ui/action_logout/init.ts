import { newClearAuthTicketInfra } from "../clear/init"

import { initLogoutResource } from "./impl"
import { initLogoutCoreAction, initLogoutCoreMaterial } from "./core/impl"

import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/feature"
import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/feature"

import { LogoutResource } from "./resource"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutResource(feature: OutsideFeature): LogoutResource {
    return initLogoutResource(
        initLogoutCoreAction(initLogoutCoreMaterial(newClearAuthTicketInfra(feature))),
    )
}
