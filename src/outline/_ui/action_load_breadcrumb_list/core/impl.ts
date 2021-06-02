import { loadBreadcrumbList } from "../../load_breadcrumb_list/impl"

import { LoadBreadcrumbListInfra } from "../../load_breadcrumb_list/infra"

import { LoadBreadcrumbListCoreAction, LoadBreadcrumbListCoreMaterial } from "./action"

import { LoadMenuLocationDetecter } from "../../kernel/method"

export function initLoadBreadcrumbListCoreMaterial(
    infra: LoadBreadcrumbListInfra,
    detecter: LoadMenuLocationDetecter,
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
