import { NotifyUnexpectedErrorAction } from "../avail/unexpected_error/notify/action"
import { LoadBreadcrumbListAction, LoadMenuAction } from "../common/outline/load/action"

export type DocsResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: LoadBreadcrumbListAction
    menu: LoadMenuAction
}>
