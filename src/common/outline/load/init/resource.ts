import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newOutlineMenuInfra } from "./infra"
import { newOutlineMenuShell, newLoadBreadcrumbListShell } from "./shell"
import { newLoadBreadcrumbListConfig, newOutlineMenuConfig } from "./config"

import {
    initLoadBreadcrumbListAction,
    initOutlineMenuAction,
    LoadBreadcrumbListAction,
    OutlineMenuAction,
} from "../action"

import { MenuContent } from "../infra"

type BreadcrumbListOutsideFeature = LocationOutsideFeature
export function newLoadBreadcrumbListResource(
    feature: BreadcrumbListOutsideFeature,
    menuContent: MenuContent,
): Readonly<{ breadcrumbList: LoadBreadcrumbListAction }> {
    return {
        breadcrumbList: initLoadBreadcrumbListAction({
            config: newLoadBreadcrumbListConfig(menuContent),
            shell: newLoadBreadcrumbListShell(feature),
        }),
    }
}

type MenuOutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature & LocationOutsideFeature
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
