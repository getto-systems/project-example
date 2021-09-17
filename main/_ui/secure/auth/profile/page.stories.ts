import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { ProfilePageComponent } from "./page"

import { mockBaseResource } from "../../../../../src/example/_ui/action_base/mock"
import { mockChangePasswordResource } from "../../../../../src/auth/password/_ui/action_change/mock"

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
        ...mockChangePasswordResource(),
    })
})

export const Profile = template({})
