import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { ProfileComponent } from "./profile"

import { mockNotifyUnexpectedErrorAction } from "../../../../avail/unexpected_error/_ui/action_notify/mock"
import {
    mockBreadcrumbList_home,
    mockLoadBreadcrumbListAction,
} from "../../../../example/outline/_ui/action_load_breadcrumb_list/mock"
import { mockLoadMenuAction, mockMenu_home } from "../../../../example/outline/_ui/action_load_menu/mock"
import { mockLoadSeasonResource } from "../../../../example/_ui/common/action_load_season/mock"
import { mockLogoutAction } from "../../../auth_ticket/_ui/action_logout/mock"

export default {
    title: "main/Auth/Profile",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = {
    // no props
}
const template = storyTemplate<MockProps>(() => {
    return h(ProfileComponent, {
        error: mockNotifyUnexpectedErrorAction(),
        breadcrumbList: mockLoadBreadcrumbListAction(mockBreadcrumbList_home()),
        menu: mockLoadMenuAction(mockMenu_home()),
        ...mockLoadSeasonResource(),
        logout: mockLogoutAction(),
    })
})

export const Profile = template({})
