import { homeMenuContent } from "../../../../x_content/menu/home_menu"

import { newNotifyUnexpectedErrorResource } from "../../../../avail/unexpected_error/notify/detail/resource"
import {
    newOutlineBreadcrumbListResource,
    newOutlineMenuResource,
} from "../../../../common/outline/load/detail/resource"
import { newLoadSeasonResource } from "../../../../core/season/load/detail/resource"

import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"
import { LocationOutsideFeature } from "../../../../common/util/location/feature"

import { BaseResource } from "../resource"

export type BaseOutsideFeature = RepositoryOutsideFeature & LocationOutsideFeature
export function newBaseResource(feature: BaseOutsideFeature): BaseResource {
    const menu = homeMenuContent()
    return {
        ...newOutlineBreadcrumbListResource(feature, menu),
        ...newOutlineMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(),
        ...newLoadSeasonResource(feature),
    }
}
