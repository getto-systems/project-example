import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { InputLoginIDComponent } from "./input"

import { mockBoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/init/mock"
import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"

import { LOGIN_ID_MAX_LENGTH } from "../convert"

import { initInputLoginIDAction, ValidateLoginIDState } from "../action"

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
    loginID: string
    validate: typeof options[number]
    help: string
}>
const template = storyTemplate<Props>((props) => {
    const { input: field } = initInputLoginIDAction()
    const store = mockBoardValueStore()
    field.input.connector.connect(store)

    store.set(markBoardValue(props.loginID))

    return h(InputLoginIDComponent, {
        field,
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

export const InputLoginID = template({
    loginID: "",
    validate: "valid",
    help: "",
})
