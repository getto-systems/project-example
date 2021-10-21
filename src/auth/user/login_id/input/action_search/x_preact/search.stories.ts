import { h } from "preact"

import { storyTemplate } from "../../../../../../../ui/vendor/storybook/preact/story"

import { SearchLoginIDComponent } from "./search"

import { mockInputLoginIDAction } from "../mock"

export default {
    title: "library/Auth/User/Login Id/Input/Search Login ID",
}

type Props = Readonly<{
    help: string
}>
const template = storyTemplate<Props>((props) => {
    return h(SearchLoginIDComponent, {
        field: mockInputLoginIDAction(),
        help: [props.help],
        state: { hasChanged: false },
    })
})

export const SearchLoginID = template({ help: "" })
