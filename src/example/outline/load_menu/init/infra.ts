import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { newAuthTicketRepository } from "../../../../auth/ticket/kernel/init/ticket_repository"
import { newMenuExpandRepository } from "./menu_expand_repository"
import { newGetMenuBadgeRemote } from "./menu_badge_remote"
import { initMenuBadgeStore, initMenuExpandStore } from "./store"

import { LoadMenuInfra } from "../action"

import { MenuContent } from "../../kernel/infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newLoadMenuInfra(feature: OutsideFeature, menuContent: MenuContent): LoadMenuInfra {
    return {
        getMenuBadgeRemote: menuContent.loadMenuBadge
            ? newGetMenuBadgeRemote(feature)
            : async () => ({ success: true, value: new Map() }),
        ticketRepository: newAuthTicketRepository(feature),
        menuExpandRepository: newMenuExpandRepository(feature, menuContent),
        menuExpandStore: initMenuExpandStore(),
        menuBadgeStore: initMenuBadgeStore(),
    }
}
