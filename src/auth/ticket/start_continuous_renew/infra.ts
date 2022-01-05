import { AuthTicketRepository, RenewAuthTicketRemote } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime, IntervalTime } from "../../../z_lib/ui/config/infra"

export type StartContinuousRenewInfra = Readonly<{
    ticketRepository: AuthTicketRepository
    renewRemote: RenewAuthTicketRemote
    clock: Clock
}>

export type StartContinuousRenewConfig = Readonly<{
    continuousRenewInterval: IntervalTime
    ticketExpire: ExpireTime
}>
