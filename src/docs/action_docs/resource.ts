import { ApplicationView } from "../../../ui/vendor/getto-application/action/action"

import { NotifyUnexpectedErrorResource } from "../../avail/_ui/action_notify_unexpected_error/resource"
import { LoadBreadcrumbListResource } from "../../outline/_ui/action_load_breadcrumb_list/resource"
import { LoadMenuResource } from "../../outline/_ui/action_load_menu/resource"

export type DocsView = ApplicationView<DocsResource>

export type DocsResource = NotifyUnexpectedErrorResource &
    LoadBreadcrumbListResource &
    LoadMenuResource
