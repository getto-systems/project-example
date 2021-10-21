import { loadBreadcrumbList } from "../load_breadcrumb_list/method"

import { LoadMenuDetecter } from "../kernel/infra"
import { LoadBreadcrumbListInfra } from "../load_breadcrumb_list/infra"

import { LoadBreadcrumbListAction, LoadBreadcrumbListMaterial } from "./action"

export function initLoadBreadcrumbListMaterial(
    infra: LoadBreadcrumbListInfra,
): LoadBreadcrumbListMaterial {
    return {
        load: loadBreadcrumbList(infra),
    }
}

export function initLoadBreadcrumbListAction(
    material: LoadBreadcrumbListMaterial,
    detecter: LoadMenuDetecter,
): LoadBreadcrumbListAction {
    return {
        load: () => material.load(detecter()),
    }
}
