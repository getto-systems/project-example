import { ApplicationView } from "../../z_vendor/getto-application/action/action"
import { NotifyUnexpectedErrorAction } from "../../avail/unexpected_error/notify/action"
import { LoadBreadcrumbListAction, LoadMenuAction } from "../../example/outline/load/action"

export type DocsView = ApplicationView<DocsResource>

export type DocsResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: LoadBreadcrumbListAction
    menu: LoadMenuAction
}>
