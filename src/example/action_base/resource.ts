import { NotifyUnexpectedErrorAction } from "../../avail/unexpected_error/notify/action"
import { LoadSeasonAction } from "../load_season/action"
import { LoadBreadcrumbListAction } from "../outline/load_breadcrumb_list/action"
import { LoadMenuAction } from "../outline/load_menu/action"

export type BaseResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: LoadBreadcrumbListAction
    menu: LoadMenuAction
    season: LoadSeasonAction
}>
