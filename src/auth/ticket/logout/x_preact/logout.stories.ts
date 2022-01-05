import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { LogoutComponent } from "./logout"

import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"

import { initLogoutAction, LogoutState } from "../action"

import { AuthTicket } from "../../kernel/data"

const options = ["initial", "failed"] as const

export default {
    title: "library/Auth/AuthTicket/Logout",
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
        logout: initLogoutAction({
            ticketRepository: initMemoryDB<AuthTicket>(),
            logoutRemote: async () => ({ success: true, value: true }),
        }),
        state: state(),
    })

    function state(): LogoutState {
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
