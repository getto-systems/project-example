import { docsMenuContent } from "../../../example/outline/_ui/kernel/init/docs"
import { newNotifyUnexpectedErrorResource } from "../../../avail/unexpected_error/_ui/action_notify/init/resource"
import { newLoadBreadcrumbListAction } from "../../../example/outline/_ui/action_load_breadcrumb_list/init/resource"
import { newLoadMenuResource } from "../../../example/outline/_ui/action_load_menu/init/resource"

import { initDocsView } from "../init"

import { RepositoryOutsideFeature } from "../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../z_details/_ui/remote/feature"
import { LocationOutsideFeature } from "../../../z_details/_ui/location/feature"

import { DocsView } from "../resource"

export function newDocsView(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
): DocsView {
    const menu = docsMenuContent()
    return initDocsView({
        breadcrumbList: newLoadBreadcrumbListAction(feature, menu),
        ...newLoadMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(feature),
    })
}
