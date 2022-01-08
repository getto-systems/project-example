import {
    mockBreadcrumbList_home,
    mockLoadBreadcrumbListAction,
} from "../outline/action_load_breadcrumb_list/mock"
import { mockLoadMenuAction, mockMenu_home } from "../outline/action_load_menu/mock"
import { mockLoadSeasonAction } from "../action_load_season/mock"

import { BaseResource } from "./resource"

export function mockBaseResource(): BaseResource {
    return {
        error: { notify: () => null },
        breadcrumbList: mockLoadBreadcrumbListAction(mockBreadcrumbList_home()),
        menu: mockLoadMenuAction(mockMenu_home()),
        season: mockLoadSeasonAction(),
    }
}
