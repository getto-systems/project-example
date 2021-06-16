import { h } from "preact"

import { enumKeys, storyTemplate } from "../../../../../../../ui/vendor/storybook/preact/story"

import { RequestResetTokenComponent } from "./request_reset_token"

import { mockRequestResetTokenResource } from "../mock"

import { ValidateBoardActionState } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/core/action"
import { RequestResetTokenCoreState } from "../core/action"
import { ValidateBoardStateEnum } from "../../../../../../../ui/vendor/getto-application/board/validate_board/data"

enum RequestEnum {
    "initial",
    "try",
    "takeLongtime",
    "validation-error",
    "invalid",
    "server-error",
    "infra-error",
}

export default {
    title: "main/public/Auth/Sign/Password/Reset/Request Token",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        request: {
            control: { type: "select", options: enumKeys(RequestEnum) },
        },
        form: {
            control: { type: "select", options: enumKeys(ValidateBoardStateEnum) },
        },
    },
}

type Props = Readonly<{
    request: keyof typeof RequestEnum
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
