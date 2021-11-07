import { h } from "preact"

import { storyTemplate } from "../../../../storybook/preact/story"

import { CheckboxBoardComponent } from "./checkbox"

import { markBoardValue } from "../../kernel/mock"

import { initMultipleInputBoardAction } from "../init"

const options = ["inline", "block"] as const

export default {
    title: "Getto/Board/Checkbox",
    argTypes: {
        block: {
            control: { type: "select", options },
        },
    },
}

type Props = {
    block: typeof options[number]
}
const template = storyTemplate<Props>((props) => {
    const { input } = initMultipleInputBoardAction()
    return h(CheckboxBoardComponent, {
        input,
        options: [
            { key: "a", value: markBoardValue("a"), label: "選択肢A" },
            { key: "b", value: markBoardValue("b"), label: "選択肢B" },
            { key: "c", value: markBoardValue("c"), label: "選択肢C" },
        ],
        block: props.block === "block",
    })
})

export const Checkbox = template({ block: "inline" })
