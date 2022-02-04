import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newLoadMenuInfra } from "./infra"
import { newLoadMenuShell, newLoadBreadcrumbListShell } from "./shell"
import { newLoadBreadcrumbListConfig, newLoadMenuConfig } from "./config"

import {
    initLoadBreadcrumbListAction,
    initLoadMenuAction,
    LoadBreadcrumbListAction,
    LoadMenuAction,
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
export function newLoadMenuResource(
    feature: MenuOutsideFeature,
    menuContent: MenuContent,
): Readonly<{ menu: LoadMenuAction }> {
    return {
        menu: initLoadMenuAction({
            infra: newLoadMenuInfra(feature, menuContent),
            shell: newLoadMenuShell(feature),
            config: newLoadMenuConfig(menuContent),
        }),
    }
}
