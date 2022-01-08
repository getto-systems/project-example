import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { SearchAuthUserAccountPagerComponent } from "./pager"

import { newSearchAuthUserAccountConfig } from "../init/config"
import { mockRemoteInfraError } from "../../../../../z_lib/ui/remote/mock"
import { initMemoryDB } from "../../../../../z_lib/ui/repository/init/memory"
import { mockSearchAuthUserAccountShell } from "../init/mock"

import { initSearchAuthUserAccountAction, SearchAuthUserAccountState } from "../action"

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
    return h(SearchAuthUserAccountPagerComponent, {
        search: initSearchAuthUserAccountAction(
            newSearchAuthUserAccountConfig(),
            {
                searchRemote: async () => mockRemoteInfraError,
                columnsRepository: initMemoryDB(),
            },
            mockSearchAuthUserAccountShell(new URL("https://example.com"), () => null),
        ),
        state: state(),
    })

    function state(): SearchAuthUserAccountState {
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
