import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { CheckAuthTicketComponent } from "./check_auth_info"

import { mockCheckAuthTicketResource } from "../mock"

import { CheckAuthTicketCoreState } from "../core/action"

const options = ["takeLongtime", "server-error", "infra-error"] as const

export default {
    title: "main/public/Auth/Sign/AuthTicket/Check",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        check: {
            control: { type: "select", options },
        },
    },
}

type Props = Readonly<{
    check: typeof options[number]
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(CheckAuthTicketComponent, {
        ...mockCheckAuthTicketResource(),
        state: state(),
    })

    function state(): CheckAuthTicketCoreState {
        switch (props.check) {
            case "takeLongtime":
                return { type: "take-longtime-to-renew" }

            case "server-error":
                return { type: "failed-to-renew", err: { type: "server-error" } }

            case "infra-error":
                return {
                    type: "failed-to-renew",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const Check = template({ check: "takeLongtime", err: "" })
