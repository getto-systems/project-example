import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { LogoutComponent } from "./logout"

import { mockLogoutResource } from "../mock"

import { LogoutCoreState } from "../core/action"

const options = ["initial", "failed"] as const

export default {
    title: "library/Auth/Sign/AuthTicket/Logout",
    argTypes: {
        logout: {
            control: { type: "select", options },
        },
    },
}

type Props = Readonly<{
    logout: typeof options[number]
    err: string
}>

const template = storyTemplate<Props>((props) => {
    return h(LogoutComponent, {
        ...mockLogoutResource(),
        state: state(),
    })

    function state(): LogoutCoreState {
        switch (props.logout) {
            case "initial":
                return { type: "initial-logout" }

            case "failed":
                return {
                    type: "repository-error",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const Logout = template({ logout: "initial", err: "" })
