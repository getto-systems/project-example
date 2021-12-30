import { newAuthProfileRepository } from "../kernel/init/profile_repository"
import { newLogoutRemote } from "./init/remote"

import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { LogoutInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutInfra(feature: OutsideFeature): LogoutInfra {
    return {
        profileRepository: newAuthProfileRepository(feature),
        logoutRemote: newLogoutRemote(feature),
    }
}
