import { MenuTree } from "../kernel/infra"

export type LoadBreadcrumbListInfra = Readonly<{
    version: string
    menuTree: MenuTree
}>
