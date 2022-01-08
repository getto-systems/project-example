import { h } from "preact"
import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"
import { initSearchLoginIDAction } from "../action"

import { SearchLoginIDComponent } from "./search"

export default {
    title: "library/Auth/User/Login Id/Input/Search Login ID",
}

type Props = Readonly<{
    title: string
    search: string
    help: string
}>
const template = storyTemplate<Props>((props) => {
    const { input: field } = initSearchLoginIDAction(markBoardValue(props.search))

    return h(SearchLoginIDComponent, {
        field,
        title: props.title,
        help: [props.help],
        state: { hasChanged: false },
    })
})

export const SearchLoginID = template({
    title: "",
    search: "",
    help: "",
})
