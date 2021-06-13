import { AuthzRepositoryPod } from "../kernel/infra"
import { Clock } from "../../../../z_details/_ui/clock/infra"
import {
    ExpireTime,
    IntervalTime,
} from "../../../../z_details/_ui/config/infra"
import { AuthnRepositoryPod, RenewAuthTicketRemotePod } from "../kernel/infra"

export type StartContinuousRenewInfra = Readonly<{
    authn: AuthnRepositoryPod
    authz: AuthzRepositoryPod
    renew: RenewAuthTicketRemotePod
    clock: Clock
    config: Readonly<{
        interval: IntervalTime
        authnExpire: ExpireTime
    }>
}>
