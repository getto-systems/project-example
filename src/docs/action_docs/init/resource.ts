import { docsMenuContent } from "../../../example/outline/kernel/init/docs"
import { newNotifyUnexpectedErrorResource } from "../../../avail/unexpected_error/notify/init/resource"
import { newLoadBreadcrumbListResource } from "../../../example/outline/load_breadcrumb_list/init/resource"
import { newLoadMenuResource } from "../../../example/outline/action_load_menu/init/resource"

import { initDocsView } from "../init"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../../z_lib/ui/location/feature"

import { DocsView } from "../resource"

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
