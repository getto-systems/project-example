import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { InputPasswordComponent } from "./input"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/init/mock"

import { PASSWORD_MAX_LENGTH } from "../convert"

import { initInputPasswordAction, ValidatePasswordState } from "../action"

const validateOptions = ["valid", "empty", "too-long"] as const

export default {
    title: "library/Auth/User/Password/Input/Input Password",
    argTypes: {
        validate: {
            control: { type: "select", options: validateOptions },
        },
    },
}

type Props = Readonly<{
    title: string
    password: string
    validate: typeof validateOptions[number]
    help: string
}>
const template = storyTemplate<Props>((props) => {
    const { input: field } = initInputPasswordAction()
    const store = mockBoardValueStore()
    field.input.connector.connect(store)

    store.set(markBoardValue(props.password))

    return h(InputPasswordComponent, {
        field,
        title: props.title,
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
})

export const InputPassword = template({
    title: "",
    password: "",
    validate: "valid",
    help: "",
})
