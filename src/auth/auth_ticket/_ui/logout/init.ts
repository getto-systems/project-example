import { newAuthzRepository } from "../kernel/infra/repository/authz"
import { newAuthnRepository } from "../kernel/infra/repository/authn"
import { newLogoutRemote } from "./infra/logout"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"

import { LogoutInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutInfra(feature: OutsideFeature): LogoutInfra {
    return {
        authn: newAuthnRepository(feature),
        authz: newAuthzRepository(feature),
        logout: newLogoutRemote(feature),
    }
}
