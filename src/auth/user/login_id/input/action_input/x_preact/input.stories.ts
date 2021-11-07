import { h } from "preact"

import { storyTemplate } from "../../../../../../../ui/vendor/storybook/preact/story"

import { InputLoginIDComponent } from "./input"

import { mockInputLoginIDAction } from "../mock"

import { LOGIN_ID_MAX_LENGTH } from "../../convert"

import { ValidateLoginIDState } from "../action"

const options = ["valid", "empty", "too-long"] as const

export default {
    title: "library/Auth/Common/Fields/Input Login ID",
    argTypes: {
        validate: {
            control: { type: "select", options },
        },
    },
}

type Props = Readonly<{
    validate: typeof options[number]
    help: string
}>
const template = storyTemplate<Props>((props) => {
    return h(InputLoginIDComponent, {
        field: mockInputLoginIDAction(),
        help: [props.help],
        state: state(),
    })

    function state(): ValidateLoginIDState {
        switch (props.validate) {
            case "valid":
                return { valid: true }

            case "empty":
                return { valid: false, err: [{ type: props.validate }] }

            case "too-long":
                return {
                    valid: false,
                    err: [{ type: props.validate, maxLength: LOGIN_ID_MAX_LENGTH }],
                }
        }
    }
})

export const InputLoginID = template({ validate: "valid", help: "" })
