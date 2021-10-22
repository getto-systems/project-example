import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchUserAccountFormComponent } from "./form"

import { mockSearchUserAccountAction } from "../mock"

import { SearchUserAccountState } from "../action"

const options = ["initial", "try", "take-longtime", "server-error", "infra-error"] as const
const changes = ["initial", "has-changed"] as const

export default {
    title: "main/Auth/User/Account/Search/Form",
    argTypes: {
        search: {
            control: { type: "select", options },
        },
        form: {
            control: { type: "select", options: changes },
        },
    },
}

export type Props = Readonly<{
    search: typeof options[number]
    form: typeof changes[number]
    err: string
}>
const template = storyTemplate<Props>((props) => {
    return h(SearchUserAccountFormComponent, {
        search: mockSearchUserAccountAction(),
        state: state(),
        observe: { hasChanged: props.form === "has-changed" },
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

export const Form = template({ search: "initial", form: "initial", err: "" })
