import { docsMenuContent } from "../../outline/_ui/kernel/init/docs"
import { newNotifyUnexpectedErrorResource } from "../../avail/unexpected_error/_ui/action_notify/init"
import { newLoadBreadcrumbListResource } from "../../outline/_ui/action_load_breadcrumb_list/init"
import { newLoadMenuResource } from "../../outline/_ui/action_load_menu/init"

import { initDocsView } from "./impl"

import { RepositoryOutsideFeature } from "../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../z_details/_ui/location/feature"

import { DocsView } from "./resource"

export function newDocsView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): DocsView {
    const menu = docsMenuContent()
    return initDocsView({
        ...newLoadBreadcrumbListResource(feature, menu),
        ...newLoadMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(feature),
    })
}
