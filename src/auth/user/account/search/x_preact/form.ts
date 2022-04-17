import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box_grow } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchLoginId } from "../../../login_id/input/x_preact/search"
import { SearchButton } from "../../../../../core/x_preact/button/search_button"
import { ClearSearchButton } from "../../../../../core/x_preact/button/clear_search_button"

import { SearchAuthUserAccountAction } from "../action"

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
        return h(SearchButton, {
            isConnecting: state.type === "try" || state.type === "take-longtime",
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.search()
        }
    }

    function clearButton(): VNode {
        return h(ClearSearchButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.clear()
        }
    }
}
