import { AuthnRepository, AuthzRepository, RenewAuthTicketRemote } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime, IntervalTime } from "../../../z_lib/ui/config/infra"

export type StartContinuousRenewInfra = Readonly<{
    authn: AuthnRepository
    authz: AuthzRepository
    renew: RenewAuthTicketRemote
    clock: Clock
    config: Readonly<{
        continuousRenewInterval: IntervalTime
        authnExpire: ExpireTime
    }>
}>
