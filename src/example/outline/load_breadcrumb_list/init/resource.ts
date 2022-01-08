import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { newLoadBreadcrumbListShell } from "./shell"
import { newLoadBreadcrumbListInfra } from "./infra"

import { LoadBreadcrumbListAction, initLoadBreadcrumbListAction } from "../action"

import { MenuContent } from "../../kernel/infra"


type OutsideFeature = LocationOutsideFeature
export function newLoadBreadcrumbListResource(
    feature: OutsideFeature,
    menuContent: MenuContent,
): Readonly<{ breadcrumbList: LoadBreadcrumbListAction }> {
    return {
        breadcrumbList: initLoadBreadcrumbListAction(
            newLoadBreadcrumbListInfra(menuContent),
            newLoadBreadcrumbListShell(feature),
        ),
    }
}
