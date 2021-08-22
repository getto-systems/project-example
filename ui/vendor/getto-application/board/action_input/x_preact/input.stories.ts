import { h } from "preact"

import { storyTemplate } from "../../../../storybook/preact/story"

import { InputBoardComponent_legacy } from "./input"

import { markBoardValue } from "../../kernel/mock"

import { mockInputBoardValueAction } from "../core/mock"

import { InputBoardValueType, inputBoardValueTypes } from "../../input/data"

export default {
    title: "Getto/Board/Input",
    argTypes: {
        inputType: {
            control: { type: "select", options: inputBoardValueTypes },
        },
    },
}

type Props = Readonly<{
    inputType: InputBoardValueType
    value: string
}>
const template = storyTemplate<Props>((props) => {
    return h(InputBoardComponent_legacy, {
        type: props.inputType,
        input: mockInputBoardValueAction(markBoardValue(props.value)),
    })
})

export const Input = template({ inputType: "text", value: "" })
