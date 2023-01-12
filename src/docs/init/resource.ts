import { docsMenuContent } from "../../x_content/menu/docs_menu"

import { newNotifyUnexpectedErrorResource } from "../../avail/unexpected_error/notify/init/resource"
import {
    newOutlineBreadcrumbListResource,
    newOutlineMenuResource,
} from "../../common/outline/load/init/resource"

import { RepositoryOutsideFeature } from "../../common/util/repository/feature"
import { LocationOutsideFeature } from "../../common/util/location/feature"

import { DocsResource } from "../resource"

type OutsideFeature = RepositoryOutsideFeature & LocationOutsideFeature
export function newDocsResource(feature: OutsideFeature): DocsResource {
    const menu = docsMenuContent()
    return {
        ...newOutlineBreadcrumbListResource(feature, menu),
        ...newOutlineMenuResource(feature, menu),
        ...newNotifyUnexpectedErrorResource(),
    }
}
