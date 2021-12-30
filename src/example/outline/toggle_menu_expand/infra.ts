import { AuthProfileRepository } from "../../../auth/ticket/kernel/infra"
import { MenuBadgeStore, MenuExpandRepository, MenuExpandStore, MenuTree } from "../kernel/infra"

export type ToggleMenuExpandInfra = Readonly<{
    version: string
    menuTree: MenuTree
    profileRepository: AuthProfileRepository
    menuExpandRepository: MenuExpandRepository
}>

export type ToggleMenuExpandStore = Readonly<{
    menuExpand: MenuExpandStore
    menuBadge: MenuBadgeStore
}>
