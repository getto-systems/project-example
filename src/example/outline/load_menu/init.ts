import { env } from "../../../y_environment/ui/env"

import { newAuthTicketRepository } from "../../../auth/ticket/kernel/init/ticket_repository"
import { newMenuExpandRepository } from "../kernel/init/menu_expand_repository"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { MenuContent } from "../kernel/infra"
import { LoadMenuInfra } from "./infra"

export function newLoadMenuInfra(
    feature: RepositoryOutsideFeature,
    menuContent: MenuContent,
): LoadMenuInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
        ticketRepository: newAuthTicketRepository(feature),
        menuExpandRepository: newMenuExpandRepository(feature, menuContent),
    }
}
