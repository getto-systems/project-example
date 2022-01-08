import { h } from "preact"
import { html } from "htm/preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchOffsetComponent } from "./offset"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"

import { initSearchOffsetAction } from "../action"

export default {
    title: "library/Remote/Search/Offset",
}

type Props = Readonly<{
    count: string
}>
const template = storyTemplate<Props>((props) => {
    const { input: field } = initSearchOffsetAction(markBoardValue(""))

    return h(SearchOffsetComponent, {
        field,
        count: props.count,
        options: [
            html`<option value="">すべて</option>`,
            html`<option value="0">1 - 1000</option>`,
            html`<option value="1000">1001 - 2000</option>`,
            html`<option value="2000">2001 - 2583</option>`,
        ],
        button: html`<button type="button">読み込み</button>`,
    })
})

export const Offset = template({ count: "全 2583件中" })
