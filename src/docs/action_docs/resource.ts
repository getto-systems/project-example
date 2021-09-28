import { ApplicationView } from "../../../ui/vendor/getto-application/action/action"

import { NotifyUnexpectedErrorResource } from "../../avail/unexpected_error/action_notify/resource"
import { LoadBreadcrumbListResource } from "../../example/outline/action_load_breadcrumb_list/resource"
import { LoadMenuResource } from "../../example/outline/action_load_menu/resource"

export type DocsView = ApplicationView<DocsResource>

export type DocsResource = NotifyUnexpectedErrorResource &
    LoadBreadcrumbListResource &
    LoadMenuResource
