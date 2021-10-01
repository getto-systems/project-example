import { auth_config } from "../../x_outside_feature/config"

import { newRenewAuthTicketRemote } from "../kernel/init/remote/renew"
import { newAuthzRepository } from "../kernel/init/repository/authz"
import { newAuthnRepository } from "../kernel/init/repository/authn"

import { newClock } from "../../../z_lib/ui/clock/init"

import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { StartContinuousRenewInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newStartContinuousRenewAuthnInfoInfra(
    feature: OutsideFeature,
): StartContinuousRenewInfra {
    return {
        authn: newAuthnRepository(feature),
        authz: newAuthzRepository(feature),
        renew: newRenewAuthTicketRemote(feature, newClock()),
        clock: newClock(),
        config: {
            authnExpire: auth_config.authnExpire,
            continuousRenewInterval: auth_config.continuousRenewInterval,
        },
    }
}