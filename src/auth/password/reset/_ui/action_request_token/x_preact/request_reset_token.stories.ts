import { h } from "preact"

import { storyTemplate } from "../../../../../../../ui/vendor/storybook/preact/story"

import { RequestResetTokenComponent } from "./request_reset_token"

import { mockRequestResetTokenResource } from "../mock"

import { ValidateBoardActionState } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/core/action"
import { RequestResetTokenCoreState } from "../core/action"
import { validateBoardStates } from "../../../../../../../ui/vendor/getto-application/board/validate_board/data"

const options = [
    "initial",
    "try",
    "takeLongtime",
    "success",
    "validation-error",
    "invalid",
    "server-error",
    "infra-error",
] as const

export default {
    title: "main/Auth/Password/Reset/Request Token",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        request: {
            control: { type: "select", options },
        },
        form: {
            control: { type: "select", options: validateBoardStates },
        },
    },
}

type Props = Readonly<{
    request: typeof options[number]
    form: ValidateBoardActionState
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(RequestResetTokenComponent, {
        ...mockRequestResetTokenResource(),
        state: {
            core: state(),
            form: props.form,
        },
    })

    function state(): RequestResetTokenCoreState {
        switch (props.request) {
            case "initial":
                return { type: "initial-request-token" }

            case "try":
                return { type: "try-to-request-token" }

            case "takeLongtime":
                return { type: "take-longtime-to-request-token" }

            case "success":
                return { type: "succeed-to-request-token" }

            case "validation-error":
                return {
                    type: "failed-to-request-token",
                    err: { type: "validation-error" },
                }

            case "invalid":
                return {
                    type: "failed-to-request-token",
                    err: { type: "invalid-reset" },
                }

            case "server-error":
                return {
                    type: "failed-to-request-token",
                    err: { type: "server-error" },
                }

            case "infra-error":
                return {
                    type: "failed-to-request-token",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const RequestToken = template({ request: "initial", form: "valid", err: "" })
