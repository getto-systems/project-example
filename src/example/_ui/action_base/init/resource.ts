import { homeMenuContent } from "../../../outline/kernel/init/home"
import { newNotifyUnexpectedErrorResource } from "../../../../avail/unexpected_error/action_notify/init/resource"
import { newLoadBreadcrumbListAction } from "../../../outline/action_load_breadcrumb_list/init/resource"
import { newLoadMenuResource } from "../../../outline/action_load_menu/init/resource"
import { newLoadSeasonResource } from "../../common/action_load_season/init/resource"

import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../../z_details/_ui/location/feature"

import { BaseResource } from "../resource"

export type BaseOutsideFeature = RemoteOutsideFeature &
    RepositoryOutsideFeature &
    LocationOutsideFeature
export function newBaseResource(feature: BaseOutsideFeature): BaseResource {
    const menu = homeMenuContent()
    return {
        breadcrumbList: newLoadBreadcrumbListAction(feature, menu),
        ...newLoadMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(feature),
        ...newLoadSeasonResource(feature),
    }
}
