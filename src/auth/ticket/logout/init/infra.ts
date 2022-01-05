import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { newAuthTicketRepository } from "../../kernel/init/ticket_repository"
import { newLogoutRemote } from "./logout_remote"

import { LogoutInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newLogoutInfra(feature: OutsideFeature): LogoutInfra {
    return {
        ticketRepository: newAuthTicketRepository(feature),
        logoutRemote: newLogoutRemote(feature),
    }
}
