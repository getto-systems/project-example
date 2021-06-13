import { AuthzRepositoryPod } from "../kernel/infra"
import { Clock } from "../../../../z_details/_ui/clock/infra"
import { DelayTime, ExpireTime } from "../../../../z_details/_ui/config/infra"
import { AuthnRepositoryPod, RenewAuthTicketRemotePod } from "../kernel/infra"

export type CheckAuthTicketInfra = Readonly<{
    authz: AuthzRepositoryPod
    authn: AuthnRepositoryPod
    renew: RenewAuthTicketRemotePod
    clock: Clock
    config: Readonly<{
        instantLoadExpire: ExpireTime
        takeLongtimeThreshold: DelayTime
    }>
}>
