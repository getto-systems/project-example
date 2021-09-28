import { h } from "preact"
import { storyTemplate } from "../../../ui/vendor/storybook/preact/story"

import { ApplicationErrorComponent } from "./application_error"

export default {
    title: "library/Avail/Common/Application Error",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = Readonly<{
    err: string
}>
const template = storyTemplate<MockProps>((args) => {
    return h(ApplicationErrorComponent, args)
})

export const ApplicationError = template({ err: "application error" })
