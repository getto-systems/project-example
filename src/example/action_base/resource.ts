import { NotifyUnexpectedErrorAction } from "../../avail/unexpected_error/notify/action"
import { LoadSeasonAction } from "../season/load/action"
import { LoadBreadcrumbListAction } from "../outline/load_breadcrumb_list/action"
import { LoadMenuAction } from "../outline/load/action"

export type BaseResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: LoadBreadcrumbListAction
    menu: LoadMenuAction
    season: LoadSeasonAction
}>
