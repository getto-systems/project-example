import { NotifyUnexpectedErrorAction } from "../../avail/unexpected_error/notify/action"
import { LoadSeasonAction } from "../../core/season/load/action"
import { LoadBreadcrumbListAction, LoadMenuAction } from "../outline/load/action"

export type BaseResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: LoadBreadcrumbListAction
    menu: LoadMenuAction
    season: LoadSeasonAction
}>
