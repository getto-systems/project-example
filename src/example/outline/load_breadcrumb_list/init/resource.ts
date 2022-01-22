import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { newLoadBreadcrumbListShell } from "./shell"
import { newLoadBreadcrumbListConfig } from "./config"

import { LoadBreadcrumbListAction, initLoadBreadcrumbListAction } from "../action"

import { MenuContent } from "../../kernel/infra"

type OutsideFeature = LocationOutsideFeature
export function newLoadBreadcrumbListResource(
    feature: OutsideFeature,
    menuContent: MenuContent,
): Readonly<{ breadcrumbList: LoadBreadcrumbListAction }> {
    return {
        breadcrumbList: initLoadBreadcrumbListAction({
            config: newLoadBreadcrumbListConfig(menuContent),
            shell: newLoadBreadcrumbListShell(feature),
        }),
    }
}
