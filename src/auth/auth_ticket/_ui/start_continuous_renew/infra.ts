import { AuthzRepositoryPod } from "../kernel/infra"
import { Clock } from "../../../../../ui/vendor/getto-application/infra/clock/infra"
import {
    ExpireTime,
    IntervalTime,
} from "../../../../../ui/vendor/getto-application/infra/config/infra"
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
