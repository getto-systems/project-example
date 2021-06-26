import { h } from "preact"

import { storyTemplate } from "../../../../../../../ui/vendor/storybook/preact/story"

import { ResetPasswordComponent } from "./reset_password"

import { mockResetPasswordResource } from "../mock"

import { ValidateBoardActionState } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/core/action"
import { ResetPasswordCoreState } from "../core/action"
import { validateBoardStates } from "../../../../../../../ui/vendor/getto-application/board/validate_board/data"

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
    title: "main/public/Auth/Sign/Password/Reset/Reset",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        reset: {
            control: { type: "select", options },
        },
        form: {
            control: { type: "select", options: validateBoardStates },
        },
    },
}

type Props = Readonly<{
    reset: typeof options[number]
    form: ValidateBoardActionState
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(ResetPasswordComponent, {
        ...mockResetPasswordResource(),
        state: { core: state(), form: props.form },
    })

    function state(): ResetPasswordCoreState {
        switch (props.reset) {
            case "initial":
                return { type: "initial-reset" }

            case "try":
                return { type: "try-to-reset" }

            case "takeLongtime":
                return { type: "take-longtime-to-reset" }

            case "validation-error":
                return { type: "failed-to-reset", err: { type: "validation-error" } }

            case "invalid":
                return {
                    type: "failed-to-reset",
                    err: { type: "invalid-reset" },
                }

            case "server-error":
                return { type: "failed-to-reset", err: { type: "server-error" } }

            case "infra-error":
                return {
                    type: "failed-to-reset",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const Reset = template({ reset: "initial", form: "valid", err: "" })
