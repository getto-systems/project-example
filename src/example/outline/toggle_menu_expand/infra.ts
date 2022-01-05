import { AuthTicketRepository } from "../../../auth/ticket/kernel/infra"
import { MenuBadgeStore, MenuExpandRepository, MenuExpandStore, MenuTree } from "../kernel/infra"

export type ToggleMenuExpandInfra = Readonly<{
    version: string
    menuTree: MenuTree
    ticketRepository: AuthTicketRepository
    menuExpandRepository: MenuExpandRepository
}>

export type ToggleMenuExpandStore = Readonly<{
    menuExpand: MenuExpandStore
    menuBadge: MenuBadgeStore
}>
