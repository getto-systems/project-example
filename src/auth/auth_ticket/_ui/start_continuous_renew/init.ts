import { newRenewAuthTicketRemote } from "../kernel/init/remote/renew"
import { newAuthzRepository } from "../kernel/init/repository/authz"
import { newAuthnRepository } from "../kernel/init/repository/authn"

import { newClock } from "../../../../z_details/_ui/clock/init"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"

import { expireMinute, intervalMinute } from "../../../../z_details/_ui/config/infra"
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
            authnExpire: expireMinute(1),
            interval: intervalMinute(2),
        },
    }
}
