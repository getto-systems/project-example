import { h } from "preact"

import { storyTemplate } from "../../../../ui/vendor/storybook/preact/story"

import { DashboardComponent } from "./dashboard"

import { mockBaseResource } from "../../action_base/mock"

export default {
    title: "main/Example/Dashboard",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = {
    // no props
}
const template = storyTemplate<MockProps>(() => {
    return h(DashboardComponent, mockBaseResource())
})

export const Dashboard = template({})
