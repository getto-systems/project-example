import { auth_config } from "../../x_outside_feature/config"

import { newAuthProfileRepository } from "../kernel/init/profile_repository"
import { newRenewAuthTicketRemote } from "../kernel/init/renew_remote"

import { newClock } from "../../../z_lib/ui/clock/init"

import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { CheckAuthTicketInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newCheckAuthTicketInfra(feature: OutsideFeature): CheckAuthTicketInfra {
    return {
        profileRepository: newAuthProfileRepository(feature),
        renewRemote: newRenewAuthTicketRemote(feature, newClock()),
        clock: newClock(),
        config: {
            instantLoadExpire: auth_config.instantLoadExpire,
            takeLongtimeThreshold: auth_config.takeLongtimeThreshold,
        },
    }
}
