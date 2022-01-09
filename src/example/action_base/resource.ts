import { ApplicationView } from "../../../ui/vendor/getto-application/action/action"
import { NotifyUnexpectedErrorAction } from "../../avail/unexpected_error/notify/action"
import { LoadSeasonAction } from "../load_season/action"
import { LoadBreadcrumbListAction } from "../outline/load_breadcrumb_list/action"
import { LoadMenuAction } from "../outline/load_menu/action"

// TODO 多分必要ない
export type BaseTypes<R> = {
    view: BaseView<R>
    resource: R & BaseResource
}

// TODO これも必要ない
export type BaseView<R> = ApplicationView<R & BaseResource>

export type BaseResource = Readonly<{
    error: NotifyUnexpectedErrorAction
    breadcrumbList: LoadBreadcrumbListAction
    menu: LoadMenuAction
    season: LoadSeasonAction
}>
