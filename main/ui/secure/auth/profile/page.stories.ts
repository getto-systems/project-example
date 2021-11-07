import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { ProfilePageComponent } from "./page"

import { mockBaseResource } from "../../../../../src/example/action_base/mock"
import { mockChangePasswordAction } from "../../../../../src/auth/user/password/action_change/mock"
import { mockRequestResetTokenProfileAction } from "../../../../../src/auth/user/password/reset/action_request_token_profile/mock"

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
    return h(ProfilePageComponent, {
        ...mockBaseResource(),
        change: mockChangePasswordAction(),
        requestToken: mockRequestResetTokenProfileAction(),
    })
})

export const Profile = template({})
