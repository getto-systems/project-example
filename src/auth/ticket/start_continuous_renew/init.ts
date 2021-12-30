import { auth_config } from "../../x_outside_feature/config"

import { newRenewAuthTicketRemote } from "../kernel/init/renew_remote"
import { newAuthProfileRepository } from "../kernel/init/profile_repository"

import { newClock } from "../../../z_lib/ui/clock/init"

import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { StartContinuousRenewInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newStartContinuousRenewAuthnInfoInfra(
    feature: OutsideFeature,
): StartContinuousRenewInfra {
    return {
        profileRepository: newAuthProfileRepository(feature),
        renewRemote: newRenewAuthTicketRemote(feature, newClock()),
        clock: newClock(),
        config: {
            authnExpire: auth_config.authnExpire,
            continuousRenewInterval: auth_config.continuousRenewInterval,
        },
    }
}
