import { docsMenuContent } from "../../x_content/menu/docs_menu"

import { newNotifyUnexpectedErrorResource } from "../../avail/unexpected_error/notify/init/resource"
import {
    newLoadBreadcrumbListResource,
    newOutlineMenuResource,
} from "../../common/outline/load/init/resource"

import { RepositoryOutsideFeature } from "../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../z_lib/ui/remote/feature"
import { LocationOutsideFeature } from "../../z_lib/ui/location/feature"

import { DocsResource } from "../resource"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature
export function newDocsResource(feature: OutsideFeature): DocsResource {
    const menu = docsMenuContent()
    return {
        ...newLoadBreadcrumbListResource(feature, menu),
        ...newOutlineMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(feature),
    }
}
