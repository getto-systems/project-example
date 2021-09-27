import { AuthzRepository } from "../../../../auth/ticket/kernel/infra"
import { MenuExpandRepository, MenuExpandStore, MenuTree } from "../kernel/infra"

export type LoadMenuInfra = Readonly<{
    version: string
    menuTree: MenuTree
    authz: AuthzRepository
    menuExpand: MenuExpandRepository
}>

export type LoadMenuStore = Readonly<{
    menuExpand: MenuExpandStore
}>
