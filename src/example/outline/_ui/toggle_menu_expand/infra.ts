import { AuthzRepository } from "../../../../auth/auth_ticket/_ui/kernel/infra"
import { MenuBadgeStore, MenuExpandRepository, MenuExpandStore, MenuTree } from "../kernel/infra"

export type ToggleMenuExpandInfra = Readonly<{
    version: string
    menuTree: MenuTree
    authz: AuthzRepository
    menuExpand: MenuExpandRepository
}>

export type ToggleMenuExpandStore = Readonly<{
    menuExpand: MenuExpandStore
    menuBadge: MenuBadgeStore
}>
