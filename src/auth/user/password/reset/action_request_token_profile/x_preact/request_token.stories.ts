import { h } from "preact"

import { storyTemplate } from "../../../../../../../ui/vendor/storybook/preact/story"

import { RequestResetTokenProfileComponent } from "./request_token"

import { mockRequestResetTokenProfileAction } from "../mock"

import { ValidateBoardActionState } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { RequestResetTokenProfileState } from "../action"
import { validateBoardStates } from "../../../../../../../ui/vendor/getto-application/board/validate_board/data"

const options = [
    "initial",
    "input",
    "try",
    "takeLongtime",
    "success",
    "validation-error",
    "invalid",
    "server-error",
    "infra-error",
] as const

export default {
    title: "main/Auth/User/Password/Reset/Request Token Profile",
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
    validate: ValidateBoardActionState
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(RequestResetTokenProfileComponent, {
        requestToken: mockRequestResetTokenProfileAction(),
        state: state(),
        validate: props.validate,
    })

    function state(): RequestResetTokenProfileState {
        switch (props.request) {
            case "initial":
                return { type: "initial-request-token" }

            case "input":
                return { type: "input-login-id" }

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

export const RequestToken = template({ request: "initial", validate: "valid", err: "" })
