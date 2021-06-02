import { LoadMenuLocationDetecter } from "../kernel/method"

import { BreadcrumbList } from "./data"

export interface LoadBreadcrumbListPod {
    (detecter: LoadMenuLocationDetecter): LoadBreadcrumbListMethod
}
export interface LoadBreadcrumbListMethod {
    (): BreadcrumbList
}
