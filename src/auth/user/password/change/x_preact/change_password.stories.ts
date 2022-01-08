import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { ChangePasswordComponent } from "./change_password"

import { mockChangePasswordAction } from "../mock"

import { ChangePasswordState } from "../action"
import { ValidateBoardActionState } from "../../../../../../ui/vendor/getto-application/board/validate_board/action"

import { validateBoardStates } from "../../../../../../ui/vendor/getto-application/board/validate_board/data"

const options = [
    "initial",
    "input",
    "try",
    "takeLongtime",
    "validation-error",
    "invalid",
    "server-error",
    "infra-error",
] as const

export default {
    title: "main/Auth/User/Password/Change",
    argTypes: {
        change: {
            control: { type: "select", options },
        },
        form: {
            control: { type: "select", options: validateBoardStates },
        },
    },
}

export type Props = Readonly<{
    change: typeof options[number]
    validate: ValidateBoardActionState
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(ChangePasswordComponent, {
        change: mockChangePasswordAction(),
        state: state(),
        validate: props.validate,
    })

    function state(): ChangePasswordState {
        switch (props.change) {
            case "initial":
                return { type: "initial-change-password" }

            case "input":
                return { type: "input-password" }

            case "try":
                return { type: "try-to-change-password" }

            case "takeLongtime":
                return { type: "take-longtime-to-change-password" }

            case "validation-error":
                return { type: "failed-to-change-password", err: { type: "validation-error" } }

            case "invalid":
                return {
                    type: "failed-to-change-password",
                    err: { type: "invalid-password" },
                }

            case "server-error":
                return { type: "failed-to-change-password", err: { type: "server-error" } }

            case "infra-error":
                return {
                    type: "failed-to-change-password",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const Change = template({ change: "initial", validate: "valid", err: "" })
