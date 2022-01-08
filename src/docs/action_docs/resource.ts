import { ApplicationView } from "../../../ui/vendor/getto-application/action/action"
import { NotifyUnexpectedErrorAction } from "../../avail/unexpected_error/notify/action"

import { LoadBreadcrumbListResource } from "../../example/outline/action_load_breadcrumb_list/resource"
import { LoadMenuResource } from "../../example/outline/action_load_menu/resource"

export type DocsView = ApplicationView<DocsResource>

export type DocsResource = Readonly<{
    error: NotifyUnexpectedErrorAction
}> &
    LoadBreadcrumbListResource &
    LoadMenuResource
