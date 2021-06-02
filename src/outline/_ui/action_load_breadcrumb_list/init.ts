import { newLoadMenuLocationDetecter } from "../kernel/impl/init"
import { newLoadBreadcrumbListInfra } from "../load_breadcrumb_list/init"

import { initLoadBreadcrumbListCoreAction, initLoadBreadcrumbListCoreMaterial } from "./core/impl"

import { MenuContent } from "../kernel/infra"

import { LoadBreadcrumbListResource } from "./resource"
import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/infra"

export function newLoadBreadcrumbListResource(
    feature: LocationOutsideFeature,
    menuContent: MenuContent,
): LoadBreadcrumbListResource {
    return {
        breadcrumbList: initLoadBreadcrumbListCoreAction(
            initLoadBreadcrumbListCoreMaterial(
                newLoadBreadcrumbListInfra(menuContent),
                newLoadMenuLocationDetecter(feature),
            ),
        ),
    }
}
