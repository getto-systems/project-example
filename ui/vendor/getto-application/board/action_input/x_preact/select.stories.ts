import { h } from "preact"
import { html } from "htm/preact"

import { storyTemplate } from "../../../../storybook/preact/story"

import { SelectBoardComponent } from "./select"

import { initInputBoardAction } from "../init"

export default {
    title: "Getto/Board/Select",
}

type Props = {
    // no props
}
const template = storyTemplate<Props>((_props) => {
    const { input } = initInputBoardAction()
    return h(SelectBoardComponent, {
        input,
        options: [
            html`<option value="">選択なし</option>`,
            html`<option value="a">選択肢A</option>`,
            html`<option value="b">選択肢B</option>`,
            html`<option value="c">選択肢C</option>`,
        ],
    })
})

export const Select = template({})
