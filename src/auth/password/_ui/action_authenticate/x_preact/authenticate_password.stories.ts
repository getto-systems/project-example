import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { AuthenticatePasswordComponent } from "./authenticate_password"

import { mockAuthenticatePasswordAction } from "../mock"

import { AuthenticatePasswordState } from "../action"
import { ValidateBoardActionState } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { validateBoardStates } from "../../../../../../ui/vendor/getto-application/board/validate_board/data"

const options = [
    "initial",
    "try",
    "takeLongtime",
    "validation-error",
    "invalid",
    "server-error",
    "infra-error",
] as const

export default {
    title: "main/Auth/Password/Authenticate",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        authenticate: {
            control: { type: "select", options },
        },
        form: {
            control: { type: "select", options: validateBoardStates },
        },
    },
}

export type Props = Readonly<{
    authenticate: typeof options[number]
    validate: ValidateBoardActionState
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(AuthenticatePasswordComponent, {
        authenticate: mockAuthenticatePasswordAction(),
        state: state(),
        validate: props.validate,
    })

    function state(): AuthenticatePasswordState {
        switch (props.authenticate) {
            case "initial":
                return { type: "initial-login" }

            case "try":
                return { type: "try-to-login" }

            case "takeLongtime":
                return { type: "take-longtime-to-login" }

            case "validation-error":
                return { type: "failed-to-login", err: { type: "validation-error" } }

            case "invalid":
                return {
                    type: "failed-to-login",
                    err: { type: "invalid-password" },
                }

            case "server-error":
                return { type: "failed-to-login", err: { type: "server-error" } }

            case "infra-error":
                return {
                    type: "failed-to-login",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const Authenticate = template({ authenticate: "initial", validate: "valid", err: "" })
