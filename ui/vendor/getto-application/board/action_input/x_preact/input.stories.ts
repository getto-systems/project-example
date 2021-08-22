import { h } from "preact"

import { storyTemplate } from "../../../../storybook/preact/story"

import { InputBoardComponent } from "./input"

import { markBoardValue } from "../../kernel/mock"
import { initInputBoardAction } from "../init"

import { InputType, inputTypes } from "./input"

export default {
    title: "Getto/Board/Input",
    argTypes: {
        inputType: {
            control: { type: "select", options: inputTypes },
        },
    },
}

type Props = Readonly<{
    inputType: InputType
    value: string
}>
const template = storyTemplate<Props>((props) => {
    const { input, store } = initInputBoardAction()
    store.set(markBoardValue(props.value))
    return h(InputBoardComponent, {
        type: props.inputType,
        input,
    })
})

export const Input = template({ inputType: "text", value: "" })
