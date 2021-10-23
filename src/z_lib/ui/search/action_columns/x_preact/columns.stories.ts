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
        field: mockSearchColumnsAction(["a", "b", "c"]),
        title: props.title,
        label: (key) => `カラム-${key}`,
        block: props.block === "block",
    })
})

export const Offset = template({ title: "表示する列", block: "inline" })
