import { AuthzRepository } from "../../../../auth/auth_ticket/_ui/kernel/infra"
import { GetMenuBadgeRemote, MenuBadgeStore, MenuExpandStore, MenuTree } from "../kernel/infra"

export type UpdateMenuBadgeInfra = Readonly<{
    version: string
    menuTree: MenuTree
    authz: AuthzRepository
    getMenuBadge: GetMenuBadgeRemote
}>

export type UpdateMenuBadgeStore = Readonly<{
    menuExpand: MenuExpandStore
    menuBadge: MenuBadgeStore
}>
