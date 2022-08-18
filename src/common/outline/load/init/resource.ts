import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

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
