import { mockNotifyUnexpectedErrorAction } from "../../../avail/unexpected_error/_ui/action_notify/mock"
import {
    mockBreadcrumbList_home,
    mockLoadBreadcrumbListAction,
} from "../../outline/_ui/action_load_breadcrumb_list/mock"
import { mockLoadMenuResource } from "../../outline/_ui/action_load_menu/mock"
import { mockLoadSeasonResource } from "../common/action_load_season/mock"

import { BaseResource } from "./resource"

export function mockBaseResource(): BaseResource {
    return {
        error: mockNotifyUnexpectedErrorAction(),
        breadcrumbList: mockLoadBreadcrumbListAction(mockBreadcrumbList_home()),
        ...mockLoadMenuResource(),
        ...mockLoadSeasonResource(),
    }
}
