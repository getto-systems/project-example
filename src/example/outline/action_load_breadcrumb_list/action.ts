import { LoadBreadcrumbListMethod } from "../load_breadcrumb_list/method"

import { BreadcrumbList } from "../load_breadcrumb_list/data"

export interface LoadBreadcrumbListAction {
    load(): BreadcrumbList
}

export type LoadBreadcrumbListMaterial = Readonly<{
    load: LoadBreadcrumbListMethod
}>
