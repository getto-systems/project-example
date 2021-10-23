import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchColumnsComponent } from "./columns"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"

import { initSearchColumnsAction } from "../init"

const options = ["inline", "block"] as const

export default {
    title: "library/Remote/Search/Offset",
    argTypes: {
        block: {
            control: { type: "select", options },
        },
    },
}

type Props = Readonly<{
    title: string
    block: typeof options[number]
}>
const template = storyTemplate<Props>((props) => {
    const { input } = initSearchColumnsAction([])
    return h(SearchColumnsComponent, {
        field: input,
        title: props.title,
        options: [
            { key: "a", value: markBoardValue("a"), label: "カラムA" },
            { key: "b", value: markBoardValue("b"), label: "カラムB" },
            { key: "c", value: markBoardValue("c"), label: "カラムC" },
        ],
        block: props.block === "block",
    })
})

export const Offset = template({ title: "表示する列", block: "inline" })
