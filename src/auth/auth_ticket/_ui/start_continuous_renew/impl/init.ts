import { newAuthzRepositoryPod } from "../../kernel/infra/repository/authz"
import { newRenewAuthTicketRemote } from "../../kernel/infra/remote/renew"
import { newAuthnRepositoryPod } from "../../kernel/infra/repository/authn"

import { newClock } from "../../../../../../ui/vendor/getto-application/infra/clock/init"

import {
    expireMinute,
    intervalMinute,
} from "../../../../../../ui/vendor/getto-application/infra/config/infra"

import { StartContinuousRenewInfra } from "../infra"
import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { RepositoryOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/repository/infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newStartContinuousRenewAuthnInfoInfra(
    feature: OutsideFeature,
): StartContinuousRenewInfra {
    return {
        authn: newAuthnRepositoryPod(feature),
        authz: newAuthzRepositoryPod(feature),
        renew: newRenewAuthTicketRemote(feature),
        clock: newClock(),
        config: {
            authnExpire: expireMinute(1),
            interval: intervalMinute(2),
        },
    }
}
