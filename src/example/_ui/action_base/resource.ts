import { ApplicationView } from "../../../../ui/vendor/getto-application/action/action"

import { NotifyUnexpectedErrorResource } from "../../../avail/unexpected_error/action_notify/resource"
import { LoadBreadcrumbListResource } from "../../outline/_ui/action_load_breadcrumb_list/resource"
import { LoadMenuResource } from "../../outline/_ui/action_load_menu/resource"
import { LoadSeasonResource } from "../common/action_load_season/resource"

// TODO 多分必要ない
export type BaseTypes<R> = {
    view: BaseView<R>
    resource: R & BaseResource
}

// TODO これも必要ない
export type BaseView<R> = ApplicationView<R & BaseResource>

export type BaseResource = NotifyUnexpectedErrorResource &
    LoadBreadcrumbListResource &
    LoadMenuResource &
    LoadSeasonResource
