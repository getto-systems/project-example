import { mockNotifyUnexpectedErrorResource } from "../../../avail/_ui/action_notify_unexpected_error/mock"
import { mockLoadBreadcrumbListResource } from "../../../outline/_ui/action_load_breadcrumb_list/mock"
import { mockLoadMenuResource } from "../../../outline/_ui/action_load_menu/mock"
import { mockLoadSeasonResource } from "../common/action_load_season/mock"

import { BaseResource } from "./resource"

export function mockBaseResource(): BaseResource {
    return {
        ...mockNotifyUnexpectedErrorResource(),
        ...mockLoadBreadcrumbListResource(),
        ...mockLoadMenuResource(),
        ...mockLoadSeasonResource(),
    }
}
