import { AuthProfileRepository } from "../../../auth/ticket/kernel/infra"
import { GetMenuBadgeRemote, MenuBadgeStore, MenuExpandStore, MenuTree } from "../kernel/infra"

export type UpdateMenuBadgeInfra = Readonly<{
    version: string
    menuTree: MenuTree
    profileRepository: AuthProfileRepository
    getMenuBadgeRemote: GetMenuBadgeRemote
}>

export type UpdateMenuBadgeStore = Readonly<{
    menuExpand: MenuExpandStore
    menuBadge: MenuBadgeStore
}>
