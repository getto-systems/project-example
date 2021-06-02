import { docsMenuContent } from "../../outline/_ui/kernel/init/docs"
import { newNotifyUnexpectedErrorResource } from "../../avail/_ui/action_notify_unexpected_error/init"
import { newLoadBreadcrumbListResource } from "../../outline/_ui/action_load_breadcrumb_list/init"
import { newLoadMenuResource } from "../../outline/_ui/action_load_menu/init"

import { initDocsView } from "./impl"

import { DocsView } from "./resource"
import { RepositoryOutsideFeature } from "../../../ui/vendor/getto-application/infra/repository/infra"
import { RemoteOutsideFeature } from "../../../ui/vendor/getto-application/infra/remote/infra"
import { LocationOutsideFeature } from "../../../ui/vendor/getto-application/location/infra"

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
