import { LoadBreadcrumbListMethod } from "../../load_breadcrumb_list/method"

import { BreadcrumbList } from "../../load_breadcrumb_list/data"

export interface LoadBreadcrumbListCoreAction {
    load(): BreadcrumbList
}

export type LoadBreadcrumbListCoreMaterial = Readonly<{
    load: LoadBreadcrumbListMethod
}>
