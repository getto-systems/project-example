import { LoadMenuDetecter } from "../kernel/method"

import { BreadcrumbList } from "./data"

export interface LoadBreadcrumbListPod {
    (detecter: LoadMenuDetecter): LoadBreadcrumbListMethod
}
export interface LoadBreadcrumbListMethod {
    (): BreadcrumbList
}
