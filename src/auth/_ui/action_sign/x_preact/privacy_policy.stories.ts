import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { PrivacyPolicyComponent } from "./privacy_policy"

import { initSignLinkResource } from "../../common/nav/action_nav/impl"

export default {
    title: "main/public/Auth/Sign/Privacy Policy",
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
