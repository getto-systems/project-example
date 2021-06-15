import { newAuthnRepository } from "../kernel/infra/repository/authn"
import { newAuthzRepositoryPod } from "../kernel/infra/repository/authz"
import { newRenewAuthTicketRemote } from "../kernel/infra/remote/renew"

import { newClock } from "../../../../z_details/_ui/clock/init"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"

import {
    delaySecond,
    expireMinute,
} from "../../../../z_details/_ui/config/infra"
import { CheckAuthTicketInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newCheckAuthTicketInfra(
    feature: OutsideFeature,
): CheckAuthTicketInfra {
    return {
        authz: newAuthzRepositoryPod(feature),
        authn: newAuthnRepository(feature),
        renew: newRenewAuthTicketRemote(feature, newClock()),
        clock: newClock(),
        config: {
            instantLoadExpire: expireMinute(3),
            takeLongtimeThreshold: delaySecond(0.5),
        },
    }
}
