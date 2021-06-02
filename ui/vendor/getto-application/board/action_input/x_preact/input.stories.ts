import { h } from "preact"

import { enumKeys, storyTemplate } from "../../../../storybook/preact/story"

import { InputBoardComponent } from "./input"

import { markBoardValue } from "../../kernel/mock"

import { mockInputBoardValueAction } from "../core/mock"

import { InputBoardValueType, InputBoardValueTypeEnum } from "../../input/data"

export default {
    title: "Getto/Board/Input",
    argTypes: {
        inputType: {
            control: { type: "select", options: enumKeys(InputBoardValueTypeEnum) },
        },
    },
}

type Props = Readonly<{
    inputType: InputBoardValueType
    value: string
}>
const template = storyTemplate<Props>((props) => {
    return h(InputBoardComponent, {
        type: props.inputType,
        input: mockInputBoardValueAction(markBoardValue(props.value)),
    })
})

export const Input = template({ inputType: "text", value: "" })
