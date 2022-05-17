import { homeMenuContent } from "../../../x_content/menu/home_menu"

import { newNotifyUnexpectedErrorResource } from "../../../avail/unexpected_error/notify/init/resource"
import {
    newLoadBreadcrumbListResource,
    newLoadMenuResource,
} from "../../outline/load/init/resource"
import { newLoadSeasonResource } from "../../../core/season/load/init/resource"

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
