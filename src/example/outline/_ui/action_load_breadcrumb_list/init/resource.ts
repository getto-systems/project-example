import { newLoadMenuLocationDetecter } from "../../kernel/init"
import { newLoadBreadcrumbListInfra } from "../../load_breadcrumb_list/init"

import { initLoadBreadcrumbListAction, initLoadBreadcrumbListMaterial } from "../init"

import { MenuContent } from "../../kernel/infra"

import { LoadBreadcrumbListAction } from "../action"
import { LocationOutsideFeature } from "../../../../../z_details/_ui/location/feature"

export function newLoadBreadcrumbListAction(
    feature: LocationOutsideFeature,
    menuContent: MenuContent,
): LoadBreadcrumbListAction {
    return initLoadBreadcrumbListAction(
        initLoadBreadcrumbListMaterial(newLoadBreadcrumbListInfra(menuContent)),
        newLoadMenuLocationDetecter(feature),
    )
}
