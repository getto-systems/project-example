import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { LogoutPageComponent } from "./entry"

import { mockNotifyUnexpectedErrorAction } from "../../../../../../src/avail/unexpected_error/_ui/action_notify/mock"
import {
    mockBreadcrumbList_home,
    mockLoadBreadcrumbListAction,
} from "../../../../../../src/example/outline/_ui/action_load_breadcrumb_list/mock"
import {
    mockLoadMenuAction,
    mockMenu_home,
} from "../../../../../../src/example/outline/_ui/action_load_menu/mock"
import { mockLoadSeasonAction } from "../../../../../../src/example/_ui/common/action_load_season/mock"
import { mockLogoutAction } from "../../../../../../src/auth/auth_ticket/_ui/action_logout/mock"

export default {
    title: "main/Auth/AuthTicket/Logout",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = {
    // no props
}
const template = storyTemplate<MockProps>(() => {
    return h(LogoutPageComponent, {
        error: mockNotifyUnexpectedErrorAction(),
        breadcrumbList: mockLoadBreadcrumbListAction(mockBreadcrumbList_home()),
        menu: mockLoadMenuAction(mockMenu_home()),
        season: mockLoadSeasonAction(),
        logout: mockLogoutAction(),
    })
})

export const Logout = template({})
