import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { newAuthTicketRepository } from "../../kernel/detail/ticket_repository"
import { newLogoutRemote } from "./logout_remote"

import { LogoutInfra } from "../action"

type OutsideFeature = RepositoryOutsideFeature
export function newLogoutInfra(feature: OutsideFeature): LogoutInfra {
    return {
        ticketRepository: newAuthTicketRepository(feature),
        logoutRemote: newLogoutRemote(),
    }
}
