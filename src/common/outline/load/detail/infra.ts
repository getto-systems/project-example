import { RepositoryOutsideFeature } from "../../../util/repository/feature"

import { newAuthTicketRepository } from "../../../../auth/ticket/kernel/detail/ticket_repository"
import { newMenuExpandRepository } from "./menu_expand_repository"
import { newLoadMenuBadgeRemote } from "./menu_badge_remote"
import { initMenuBadgeStore, initMenuExpandStore } from "./store"

import { OutlineMenuInfra } from "../action"

import { MenuContent } from "../infra"

type OutsideFeature = RepositoryOutsideFeature
export function newOutlineMenuInfra(
    feature: OutsideFeature,
    menuContent: MenuContent,
): OutlineMenuInfra {
    return {
        loadMenuBadgeRemote: menuContent.loadMenuBadge
            ? newLoadMenuBadgeRemote()
            : async () => ({ success: true, value: new Map() }),
        ticketRepository: newAuthTicketRepository(feature),
        menuExpandRepository: newMenuExpandRepository(feature, menuContent),
        menuExpandStore: initMenuExpandStore(),
        menuBadgeStore: initMenuBadgeStore(),
    }
}
