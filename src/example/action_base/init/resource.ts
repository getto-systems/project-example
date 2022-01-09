import { homeMenuContent } from "../../outline/menu/menu_home"
import { newNotifyUnexpectedErrorResource } from "../../../avail/unexpected_error/notify/init/resource"
import { newLoadBreadcrumbListResource } from "../../outline/load_breadcrumb_list/init/resource"
import { newLoadMenuResource } from "../../outline/load_menu/init/resource"
import { newLoadSeasonResource } from "../../action_load_season/init/resource"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"

import { BaseResource } from "../resource"

export type BaseOutsideFeature = RemoteOutsideFeature &
    RepositoryOutsideFeature &
    LocationOutsideFeature
export function newBaseResource(feature: BaseOutsideFeature): BaseResource {
    const menu = homeMenuContent()
    return {
        ...newLoadBreadcrumbListResource(feature, menu),
        ...newLoadMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(feature),
        ...newLoadSeasonResource(feature),
    }
}
