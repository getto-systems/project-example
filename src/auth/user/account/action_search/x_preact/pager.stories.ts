import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchUserAccountPagerComponent } from "./pager"

import { mockSearchUserAccountAction } from "../mock"

import { SearchUserAccountState } from "../action"

const options = ["initial", "try", "take-longtime", "server-error", "infra-error"] as const

export default {
    title: "main/Auth/User/Account/Search/Pager",
    argTypes: {
        search: {
            control: { type: "select", options },
        },
    },
}

export type Props = Readonly<{
    search: typeof options[number]
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(SearchUserAccountPagerComponent, {
        search: mockSearchUserAccountAction(),
        state: state(),
    })

    function state(): SearchUserAccountState {
        switch (props.search) {
            case "initial":
                return { type: "initial-search" }

            case "try":
                return { type: "try-to-search" }

            case "take-longtime":
                return { type: "take-longtime-to-search" }

            case "server-error":
                return { type: "failed-to-search", err: { type: "server-error" } }

            case "infra-error":
                return {
                    type: "failed-to-search",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const Pager = template({ search: "initial", err: "" })
