import { AuthzRepositoryPod } from "../../../auth/auth_ticket/_ui/kernel/infra"
import { MenuExpandRepositoryPod, MenuExpandStore, MenuTree } from "../kernel/infra"

export type LoadMenuInfra = Readonly<{
    version: string
    menuTree: MenuTree
    authz: AuthzRepositoryPod
    menuExpand: MenuExpandRepositoryPod
}>

export type LoadMenuStore = Readonly<{
    menuExpand: MenuExpandStore
}>
