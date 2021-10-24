import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { useSearchUserAccountTableStructure } from "./structure"
import { SearchUserAccountColumnsComponent } from "./columns"

import { mockSearchUserAccountAction } from "../mock"

import { toSearchColumns } from "../../../../../z_lib/ui/search/columns/convert"

import { SearchColumnsState } from "../../../../../z_lib/ui/search/action_columns/action"

const options = ["success", "failed"] as const

export default {
    title: "main/Auth/User/Account/Search/Columns",
    argTypes: {
        columns: {
            control: { type: "select", options },
        },
    },
}

export type Props = Readonly<{
    columns: typeof options[number]
    err: string
}>
const template = storyTemplate<Props>((props) => {
    const search = mockSearchUserAccountAction()
    const structure = useSearchUserAccountTableStructure(search)

    return h(SearchUserAccountColumnsComponent, {
        search,
        structure,
        state: {
            type: "succeed-to-search",
            response: {
                page: { all: 0, offset: 0, limit: 0 },
                summary: {},
                users: [],
            },
        },
        columns: columns(),
    })

    function columns(): SearchColumnsState {
        switch (props.columns) {
            case "success":
                return {
                    type: "succeed-to-load",
                    columns: toSearchColumns(["granted-roles"]),
                }

            case "failed":
                return {
                    type: "repository-error",
                    err: { type: "infra-error", err: props.err },
                }
        }
    }
})

export const Columns = template({ columns: "success", err: "error" })
