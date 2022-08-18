import { NotifyUnexpectedErrorAction } from "../avail/unexpected_error/notify/action"
import { LoadBreadcrumbListAction, OutlineMenuAction } from "../common/outline/load/action"

export type DocsResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: LoadBreadcrumbListAction
    menu: OutlineMenuAction
}>
