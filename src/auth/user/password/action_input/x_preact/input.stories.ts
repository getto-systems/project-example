import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { InputPasswordComponent } from "./input"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"

import { mockInputPasswordAction } from "../mock"

import { PASSWORD_MAX_LENGTH } from "../../input/convert"

import { ValidatePasswordState } from "../action"

import { PasswordCharacterState } from "../../input/data"

const validateOptions = ["valid", "empty", "too-long"] as const
const characterOptions = ["singleByte", "multiByte"] as const

export default {
    title: "library/Auth/Common/Fields/Input Password",
    argTypes: {
        validate: {
            control: { type: "select", options: validateOptions },
        },
        character: {
            control: { type: "select", options: characterOptions },
        },
    },
}

type Props = Readonly<{
    password: string
    validate: typeof validateOptions[number]
    character: typeof characterOptions[number]
    help: string
}>
const template = storyTemplate<Props>((props) => {
    return h(InputPasswordComponent, {
        field: mockInputPasswordAction(markBoardValue(props.password), characterState()),
        help: [props.help],
        state: validateState(),
    })

    function validateState(): ValidatePasswordState {
        switch (props.validate) {
            case "valid":
                return { valid: true }

            case "empty":
                return { valid: false, err: [{ type: props.validate }] }

            case "too-long":
                return {
                    valid: false,
                    err: [{ type: props.validate, maxLength: PASSWORD_MAX_LENGTH }],
                }
        }
    }
    function characterState(): PasswordCharacterState {
        return { multiByte: props.character === "multiByte" }
    }
})

export const InputPassword = template({
    password: "",
    validate: "valid",
    character: "singleByte",
    help: "",
})
