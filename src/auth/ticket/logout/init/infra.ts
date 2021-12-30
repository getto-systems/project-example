import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { newAuthProfileRepository } from "../../kernel/init/profile_repository"
import { newLogoutRemote } from "./logout_remote"

import { LogoutInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutInfra(feature: OutsideFeature): LogoutInfra {
    return {
        profileRepository: newAuthProfileRepository(feature),
        logoutRemote: newLogoutRemote(feature),
    }
}
