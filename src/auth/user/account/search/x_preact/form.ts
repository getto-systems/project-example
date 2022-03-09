import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_search,
    button_undo,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box_grow } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchLoginIdComponent } from "../../../login_id/input/x_preact/search"

import { SearchAuthUserAccountAction, SearchAuthUserAccountState } from "../action"
import { ObserveBoardActionState } from "../../../../../z_vendor/getto-application/board/observe_board/action"
import {
    SEARCH_BUTTON_CONNECT,
    SEARCH_BUTTON_STATIC,
} from "../../../../../core/x_preact/design/table"

type EntryProps = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountFormEntry({ search }: EntryProps): VNode {
    return h(SearchAuthUserAccountFormComponent, {
        search,
        state: useApplicationAction(search),
        observe: useApplicationAction(search.observe),
    })
}

type Props = EntryProps &
    Readonly<{
        state: SearchAuthUserAccountState
        observe: ObserveBoardActionState
    }>
export function SearchAuthUserAccountFormComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, observe }: Props): VNode {
        if (observe.hasChanged) {
            return searchForm({ type: "has-changed" })
        }

        switch (state.type) {
            case "initial-search":
            case "succeed-to-search":
                return searchForm({ type: "initial" })

            case "try-to-search":
            case "take-longtime-to-search":
                return searchForm({ type: "connecting" })

            case "failed-to-search":
                return searchForm({ type: "failed" })
        }
    }

    type Content =
        | Readonly<{ type: "has-changed" }>
        | Readonly<{ type: "initial" }>
        | Readonly<{ type: "connecting" }>
        | Readonly<{ type: "failed" }>

    function searchForm(content: Content): VNode {
        return box_grow({
            body: [
                h(SearchLoginIdComponent, { field: props.search.loginId }),
                // TODO granted role (checkbox)
            ],
            footer: buttons({ left: button(), right: clearButton() }),
            form: true,
        })

        function clearButton(): VNode {
            const label = "検索項目をクリア"

            return button_undo({ label, onClick })

            function onClick(e: Event) {
                e.preventDefault()
                props.search.clear()
            }
        }

        function button(): VNode {
            switch (content.type) {
                case "has-changed":
                    return button_search({ state: "confirm", label: SEARCH_BUTTON_STATIC, onClick })

                case "initial":
                    return button_search({ state: "normal", label: SEARCH_BUTTON_STATIC, onClick })

                case "connecting":
                    return button_search({ state: "connect", label: SEARCH_BUTTON_CONNECT })

                case "failed":
                    return button_search({ state: "confirm", label: SEARCH_BUTTON_STATIC, onClick })
            }

            function onClick(e: Event) {
                e.preventDefault()
                props.search.search()
            }
        }
    }
}
