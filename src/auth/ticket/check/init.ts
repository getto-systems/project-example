import { auth_config } from "../../_ui/x_outside_feature/config"

import { newAuthnRepository } from "../kernel/init/repository/authn"
import { newAuthzRepository } from "../kernel/init/repository/authz"
import { newRenewAuthTicketRemote } from "../kernel/init/remote/renew"

import { newClock } from "../../../z_details/_ui/clock/init"

import { RemoteOutsideFeature } from "../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../z_details/_ui/repository/feature"

import { CheckAuthTicketInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newCheckAuthTicketInfra(feature: OutsideFeature): CheckAuthTicketInfra {
    return {
        authz: newAuthzRepository(feature),
        authn: newAuthnRepository(feature),
        renew: newRenewAuthTicketRemote(feature, newClock()),
        clock: newClock(),
        config: {
            instantLoadExpire: auth_config.instantLoadExpire,
            takeLongtimeThreshold: auth_config.takeLongtimeThreshold,
        },
    }
}
