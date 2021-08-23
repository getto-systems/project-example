import { loadBreadcrumbList } from "../load_breadcrumb_list/method"
import { LoadMenuDetecter } from "../kernel/method"

import { LoadBreadcrumbListInfra } from "../load_breadcrumb_list/infra"

import { LoadBreadcrumbListAction, LoadBreadcrumbListMaterial } from "./action"

export function initLoadBreadcrumbListMaterial(
    infra: LoadBreadcrumbListInfra,
    detecter: LoadMenuDetecter,
): LoadBreadcrumbListMaterial {
    return {
        load: loadBreadcrumbList(infra)(detecter),
    }
}

export function initLoadBreadcrumbListAction(
    material: LoadBreadcrumbListMaterial,
): LoadBreadcrumbListAction {
    return {
        load: material.load,
    }
}
