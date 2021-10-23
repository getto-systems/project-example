import { h } from "preact"
import { html } from "htm/preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchOffsetComponent } from "./offset"

import { mockSearchOffsetAction } from "../mock"

export default {
    title: "library/Remote/Search/Offset",
}

type Props = Readonly<{
    title: string
}>
const template = storyTemplate<Props>((props) => {
    return h(SearchOffsetComponent, {
        field: mockSearchOffsetAction(),
        title: props.title,
        options: [
            html`<option value="">すべて</option>`,
            html`<option value="0">1 - 1000</option>`,
            html`<option value="1000">1001 - 2000</option>`,
            html`<option value="2000">2001 - 2583</option>`,
        ],
        button: html`<button type="button">読み込み</button>`,
    })
})

export const Offset = template({ title: "全 2583件中" })
