import { h } from "preact"

import { enumKeys, storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { AuthenticatePasswordComponent } from "./authenticate_password"

import { mockAuthenticatePasswordResource } from "../mock"

import { AuthenticatePasswordCoreState } from "../core/action"
import { ValidateBoardActionState } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/core/action"
import { ValidateBoardStateEnum } from "../../../../../../ui/vendor/getto-application/board/validate_board/data"

enum AuthenticateEnum {
    "initial",
    "try",
    "takeLongtime",
    "validation-error",
    "invalid",
    "server-error",
    "infra-error",
}

export default {
    title: "main/public/Auth/Sign/Password/Authenticate",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        authenticate: {
            control: { type: "select", options: enumKeys(AuthenticateEnum) },
        },
        form: {
            control: { type: "select", options: enumKeys(ValidateBoardStateEnum) },
        },
    },
}

export type Props = Readonly<{
    authenticate: keyof typeof AuthenticateEnum
    form: ValidateBoardActionState
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(AuthenticatePasswordComponent, {
        ...mockAuthenticatePasswordResource(),
        state: { core: state(), form: props.form },
    })

    function state(): AuthenticatePasswordCoreState {
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

export const Authenticate = template({ authenticate: "initial", form: "valid", err: "" })
