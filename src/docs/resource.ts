import { NotifyUnexpectedErrorAction } from "../avail/unexpected_error/notify/action"
import { OutlineBreadcrumbListAction, OutlineMenuAction } from "../common/outline/load/action"

export type DocsResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: OutlineBreadcrumbListAction
    menu: OutlineMenuAction
}>
