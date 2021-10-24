import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchColumnsComponent } from "./columns"

import { mockSearchColumnsAction } from "../mock"

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
    return h(SearchColumnsComponent, {
        field: mockSearchColumnsAction(),
        title: props.title,
        block: props.block === "block",
        columns: [
            { key: "a", content: "カラムA", isVisible: true },
            { key: "b", content: "カラムB", isVisible: true },
            { key: "c", content: "カラムC", isVisible: false },
        ],
    })
})

export const Offset = template({ title: "表示する列", block: "inline" })
