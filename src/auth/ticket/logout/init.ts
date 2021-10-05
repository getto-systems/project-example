import { newAuthzRepository } from "../kernel/init/repository/authz"
import { newAuthnRepository } from "../kernel/init/repository/authn"
import { newLogoutRemote } from "./init/remote"

import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { LogoutInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutInfra(feature: OutsideFeature): LogoutInfra {
    return {
        authn: newAuthnRepository(feature),
        authz: newAuthzRepository(feature),
        logout: newLogoutRemote(feature),
    }
}
