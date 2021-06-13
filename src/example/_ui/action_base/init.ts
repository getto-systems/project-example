import { homeMenuContent } from "../../../outline/_ui/kernel/init/home"
import { newNotifyUnexpectedErrorResource } from "../../../avail/_ui/action_notify_unexpected_error/init"
import { newLoadBreadcrumbListResource } from "../../../outline/_ui/action_load_breadcrumb_list/init"
import { newLoadMenuResource } from "../../../outline/_ui/action_load_menu/init"
import { newLoadSeasonResource } from "../common/action_load_season/init"

import { RepositoryOutsideFeature } from "../../../../ui/vendor/getto-application/infra/repository/infra"
import { RemoteOutsideFeature } from "../../../../ui/vendor/getto-application/infra/remote/feature"
import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/infra"

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
