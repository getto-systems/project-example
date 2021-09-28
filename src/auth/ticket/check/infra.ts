import { AuthnRepository, AuthzRepository, RenewAuthTicketRemote } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { DelayTime, ExpireTime } from "../../../z_lib/ui/config/infra"

export type CheckAuthTicketInfra = Readonly<{
    authz: AuthzRepository
    authn: AuthnRepository
    renew: RenewAuthTicketRemote
    clock: Clock
    config: Readonly<{
        instantLoadExpire: ExpireTime
        takeLongtimeThreshold: DelayTime
    }>
}>
