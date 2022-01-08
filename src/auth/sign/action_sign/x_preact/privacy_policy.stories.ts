import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { PrivacyPolicyComponent } from "./privacy_policy"

import { initSignLinkResource } from "../../nav/init"

export default {
    title: "main/Auth/Sign/Privacy Policy",
    parameters: {
        layout: "fullscreen",
    },
}

export type Props = {
    // no props
}
const template = storyTemplate<Props>(() => {
    return h(PrivacyPolicyComponent, initSignLinkResource())
})

export const PrivacyPolicy = template({})
