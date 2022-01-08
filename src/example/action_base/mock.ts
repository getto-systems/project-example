import { mockLoadMenuAction, mockMenu_home } from "../outline/action_load_menu/mock"
import { mockLoadSeasonAction } from "../action_load_season/mock"
import { mockBreadcrumbList } from "../outline/load_breadcrumb_list/init/mock"

import { BaseResource } from "./resource"

export function mockBaseResource(): BaseResource {
    return {
        error: { notify: () => null },
        breadcrumbList: { load: () => mockBreadcrumbList("ホーム") },
        menu: mockLoadMenuAction(mockMenu_home()),
        season: mockLoadSeasonAction(),
    }
}
