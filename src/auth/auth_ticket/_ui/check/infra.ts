import { AuthzRepositoryPod } from "../kernel/infra"
import { Clock } from "../../../../../ui/vendor/getto-application/infra/clock/infra"
import { DelayTime, ExpireTime } from "../../../../../ui/vendor/getto-application/infra/config/infra"
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
