import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { NotFoundComponent } from "./not_found"

import { initGetCurrentVersionResource } from "../../../version/action_get_current/impl"
import { initGetCurrentVersionCoreAction } from "../../../version/action_get_current/core/impl"

export default {
    title: "main/public/Avail/Not Found",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = {
    version: string
}
const template = storyTemplate<MockProps>((props) => {
    return h(
        NotFoundComponent,
        initGetCurrentVersionResource(
            initGetCurrentVersionCoreAction({
                version: props.version,
            }),
        ),
    )
})

export const NotFound = template({ version: "1.0.0" })
