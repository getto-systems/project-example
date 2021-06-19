import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { NotFoundComponent } from "./not_found"

export default {
    title: "main/public/Avail/Not Found",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = {
    // no props
}
const template = storyTemplate<MockProps>((_props) => {
    return h(NotFoundComponent, {})
})

export const NotFound = template({})
