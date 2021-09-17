import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { LogoutPageComponent } from "./page"

import { mockBaseResource } from "../../../../../../src/example/_ui/action_base/mock"
import { mockLogoutResource } from "../../../../../../src/auth/auth_ticket/_ui/action_logout/mock"

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
        ...mockBaseResource(),
        ...mockLogoutResource(),
    })
})

export const Logout = template({})
