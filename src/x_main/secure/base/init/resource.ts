import { homeMenuContent } from "../../../../x_content/menu/home_menu"

import { newNotifyUnexpectedErrorResource } from "../../../../avail/unexpected_error/notify/init/resource"
import {
    newLoadBreadcrumbListResource,
    newOutlineMenuResource,
} from "../../../../common/outline/load/init/resource"
import { newLoadSeasonResource } from "../../../../core/season/load/init/resource"

import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"

import { BaseResource } from "../resource"

export type BaseOutsideFeature = RemoteOutsideFeature &
    RepositoryOutsideFeature &
    LocationOutsideFeature
export function newBaseResource(feature: BaseOutsideFeature): BaseResource {
    const menu = homeMenuContent()
    return {
        ...newLoadBreadcrumbListResource(feature, menu),
        ...newOutlineMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(feature),
        ...newLoadSeasonResource(feature),
    }
}
