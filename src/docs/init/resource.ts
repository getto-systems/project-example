import { docsMenuContent } from "../../x_content/menu/docs_menu"

import { newNotifyUnexpectedErrorResource } from "../../avail/unexpected_error/notify/init/resource"
import {
    newLoadBreadcrumbListResource,
    newLoadMenuResource,
} from "../../core/outline/load/init/resource"

import { initDocsView } from "../init"

import { RepositoryOutsideFeature } from "../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../z_lib/ui/location/feature"

import { ApplicationView } from "../../z_vendor/getto-application/action/action"
import { DocsResource } from "../resource"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature
export function newDocsView(feature: OutsideFeature): ApplicationView<DocsResource> {
    const menu = docsMenuContent()
    return initDocsView({
        ...newLoadBreadcrumbListResource(feature, menu),
        ...newLoadMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(feature),
    })
}
