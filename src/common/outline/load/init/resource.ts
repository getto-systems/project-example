import { LocationOutsideFeature } from "../../../util/location/feature"
import { RepositoryOutsideFeature } from "../../../util/repository/feature"

import { newOutlineMenuInfra } from "./infra"
import { newOutlineMenuShell, newOutlineBreadcrumbListShell } from "./shell"
import { newOutlineBreadcrumbListConfig, newOutlineMenuConfig } from "./config"

import {
    initOutlineBreadcrumbListAction,
    initOutlineMenuAction,
    OutlineBreadcrumbListAction,
    OutlineMenuAction,
} from "../action"

import { MenuContent } from "../infra"

type BreadcrumbListOutsideFeature = LocationOutsideFeature
export function newOutlineBreadcrumbListResource(
    feature: BreadcrumbListOutsideFeature,
    menuContent: MenuContent,
): Readonly<{ breadcrumbList: OutlineBreadcrumbListAction }> {
    return {
        breadcrumbList: initOutlineBreadcrumbListAction({
            config: newOutlineBreadcrumbListConfig(menuContent),
            shell: newOutlineBreadcrumbListShell(feature),
        }),
    }
}

type MenuOutsideFeature = RepositoryOutsideFeature & LocationOutsideFeature
export function newOutlineMenuResource(
    feature: MenuOutsideFeature,
    menuContent: MenuContent,
): Readonly<{ menu: OutlineMenuAction }> {
    return {
        menu: initOutlineMenuAction({
            infra: newOutlineMenuInfra(feature, menuContent),
            shell: newOutlineMenuShell(feature),
            config: newOutlineMenuConfig(menuContent),
        }),
    }
}
