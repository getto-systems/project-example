import { AuthProfileRepository } from "../../../auth/ticket/kernel/infra"
import { MenuExpandRepository, MenuExpandStore, MenuTree } from "../kernel/infra"

export type LoadMenuInfra = Readonly<{
    version: string
    menuTree: MenuTree
    profileRepository: AuthProfileRepository
    menuExpandRepository: MenuExpandRepository
}>

export type LoadMenuStore = Readonly<{
    menuExpand: MenuExpandStore
}>
