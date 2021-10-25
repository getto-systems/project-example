import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { ManageUserAccountPageComponent } from "./page"

import { mockBaseResource } from "../../../../../../src/example/action_base/mock"
import { mockSearchAuthUserAccountAction } from "../../../../../../src/auth/user/account/action_search/mock"

export default {
    title: "main/Auth/User/Account",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = {
    // no props
}
const template = storyTemplate<MockProps>(() => {
    return h(ManageUserAccountPageComponent, {
        ...mockBaseResource(),
        search: mockSearchAuthUserAccountAction(),
    })
})

export const Account = template({})
