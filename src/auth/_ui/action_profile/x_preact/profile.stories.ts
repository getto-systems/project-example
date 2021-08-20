import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { ProfileComponent } from "./profile"

import { mockNotifyUnexpectedErrorResource } from "../../../../avail/unexpected_error/_ui/action_notify_unexpected_error/mock"
import { mockLoadBreadcrumbListResource } from "../../../../outline/_ui/action_load_breadcrumb_list/mock"
import { mockLoadMenuResource } from "../../../../outline/_ui/action_load_menu/mock"
import { mockLoadSeasonResource } from "../../../../example/_ui/common/action_load_season/mock"
import { mockLogoutResource } from "../../../auth_ticket/_ui/action_logout/mock"

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
        ...mockNotifyUnexpectedErrorResource(),
        ...mockLoadBreadcrumbListResource(),
        ...mockLoadMenuResource(),
        ...mockLoadSeasonResource(),
        ...mockLogoutResource(),
    })
})

export const Profile = template({})
