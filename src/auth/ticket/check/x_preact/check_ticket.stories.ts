import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { CheckAuthTicketComponent } from "./check_ticket"

import { newCheckAuthTicketConfig } from "../init/config"

import { mockRemoteInfraError } from "../../../../z_lib/ui/remote/mock"
import { mockGetScriptPathShell } from "../../../sign/get_script_path/init/mock"
import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"
import { newClock } from "../../../../z_lib/ui/clock/init"

import { CheckAuthTicketState, initCheckAuthTicketAction } from "../action"

import { AuthTicket } from "../../kernel/data"

const options = ["takeLongtime", "server-error", "infra-error"] as const

export default {
    title: "main/Auth/AuthTicket/Check",
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
        check: initCheckAuthTicketAction(
            newCheckAuthTicketConfig(),
            {
                ticketRepository: initMemoryDB<AuthTicket>(),
                renewRemote: async () => mockRemoteInfraError,
                clock: newClock(),
            },
            {
                ...mockGetScriptPathShell(new URL("https://example.com")),
            },
        ),
        state: state(),
    })

    function state(): CheckAuthTicketState {
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
