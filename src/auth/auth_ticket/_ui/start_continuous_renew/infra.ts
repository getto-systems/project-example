import { AuthnRepository, AuthzRepositoryPod, RenewAuthTicketRemote } from "../kernel/infra"
import { Clock } from "../../../../z_details/_ui/clock/infra"
import { ExpireTime, IntervalTime } from "../../../../z_details/_ui/config/infra"

export type StartContinuousRenewInfra = Readonly<{
    authn: AuthnRepository
    authz: AuthzRepositoryPod
    renew: RenewAuthTicketRemote
    clock: Clock
    config: Readonly<{
        interval: IntervalTime
        authnExpire: ExpireTime
    }>
}>
