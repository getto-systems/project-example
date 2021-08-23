import { homeMenuContent } from "../../outline/_ui/kernel/init/home"
import { newNotifyUnexpectedErrorResource } from "../../../avail/unexpected_error/_ui/action_notify/init/resource"
import { newLoadBreadcrumbListResource } from "../../outline/_ui/action_load_breadcrumb_list/init"
import { newLoadMenuResource } from "../../outline/_ui/action_load_menu/init"
import { newLoadSeasonResource } from "../common/action_load_season/init"

import { RepositoryOutsideFeature } from "../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../z_details/_ui/location/feature"

import { BaseResource } from "./resource"

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
