import { loadBreadcrumbList } from "../../load_breadcrumb_list/method"
import { LoadMenuDetecter } from "../../kernel/method"

import { LoadBreadcrumbListInfra } from "../../load_breadcrumb_list/infra"

import { LoadBreadcrumbListCoreAction, LoadBreadcrumbListCoreMaterial } from "./action"

export function initLoadBreadcrumbListCoreMaterial(
    infra: LoadBreadcrumbListInfra,
    detecter: LoadMenuDetecter,
): LoadBreadcrumbListCoreMaterial {
    return {
        load: loadBreadcrumbList(infra)(detecter),
    }
}

export function initLoadBreadcrumbListCoreAction(
    material: LoadBreadcrumbListCoreMaterial,
): LoadBreadcrumbListCoreAction {
    return {
        load: material.load,
    }
}
