import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    buttons,
    button_search,
    button_undo,
} from "../../../../../z_vendor/getto-css/preact/design/form"
import { box_grow } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchLoginId } from "../../../login_id/input/x_preact/search"

import { SearchAuthUserAccountAction } from "../action"
import {
    SEARCH_BUTTON_CONNECT,
    SEARCH_BUTTON_STATIC,
} from "../../../../../core/x_preact/design/table"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountForm(props: Props): VNode {
    const state = useApplicationAction(props.search)
    const observeState = useApplicationAction(props.search.observe)

    return box_grow({
        body: [
            h(SearchLoginId, { field: props.search.loginId }),
            // TODO granted role (checkbox)
        ],
        footer: buttons({ left: searchButton(), right: clearButton() }),
        form: true,
    })

    function searchButton(): VNode {
        if (observeState.hasChanged) {
            return button_search({ state: "confirm", label: SEARCH_BUTTON_STATIC, onClick })
        }

        switch (state.type) {
            case "initial":
            case "success":
                return button_search({ state: "normal", label: SEARCH_BUTTON_STATIC, onClick })

            case "try":
            case "take-longtime":
                return button_search({ state: "connect", label: SEARCH_BUTTON_CONNECT })

            case "failed":
                return button_search({ state: "confirm", label: SEARCH_BUTTON_STATIC, onClick })
        }

        function onClick(e: Event) {
            e.preventDefault()
            props.search.search()
        }
    }

    function clearButton(): VNode {
        const label = "検索項目をクリア"

        return button_undo({ label, onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.clear()
        }
    }
}
