import { AuthnRepository, AuthzRepositoryPod, RenewAuthTicketRemote } from "../kernel/infra"
import { Clock } from "../../../../z_details/_ui/clock/infra"
import { DelayTime, ExpireTime } from "../../../../z_details/_ui/config/infra"

export type CheckAuthTicketInfra = Readonly<{
    authz: AuthzRepositoryPod
    authn: AuthnRepository
    renew: RenewAuthTicketRemote
    clock: Clock
    config: Readonly<{
        instantLoadExpire: ExpireTime
        takeLongtimeThreshold: DelayTime
    }>
}>
