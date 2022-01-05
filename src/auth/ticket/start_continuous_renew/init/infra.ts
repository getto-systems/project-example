import { newRenewAuthTicketRemote } from "../../kernel/init/renew_remote"
import { newAuthTicketRepository } from "../../kernel/init/ticket_repository"

import { newClock } from "../../../../z_lib/ui/clock/init"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { StartContinuousRenewInfra } from "../infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newStartContinuousRenewInfra(feature: OutsideFeature): StartContinuousRenewInfra {
    return {
        ticketRepository: newAuthTicketRepository(feature),
        renewRemote: newRenewAuthTicketRemote(feature, newClock()),
        clock: newClock(),
    }
}
