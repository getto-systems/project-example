import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchUserAccountColumnsComponent } from "./columns"

import { mockSearchUserAccountAction } from "../mock"

export default {
    title: "main/Auth/User/Account/Search/Columns",
}

export type Props = Readonly<{
    // no props
}>
const template = storyTemplate<Props>((_props) => {
    return h(SearchUserAccountColumnsComponent, {
        search: mockSearchUserAccountAction(),
        columns: [
            { key: "a", content: "カラムA", isVisible: true },
            { key: "b", content: "カラムB", isVisible: true },
            { key: "c", content: "カラムC", isVisible: false },
        ],
    })
})

export const Columns = template({})
