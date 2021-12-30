import { AuthProfileRepository, RenewAuthTicketRemote } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime, IntervalTime } from "../../../z_lib/ui/config/infra"

export type StartContinuousRenewInfra = Readonly<{
    profileRepository: AuthProfileRepository
    renewRemote: RenewAuthTicketRemote
    clock: Clock
    config: Readonly<{
        continuousRenewInterval: IntervalTime
        authnExpire: ExpireTime
    }>
}>
