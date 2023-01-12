import { h, VNode } from "preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { buttons } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { LoginIdFilter } from "../../../login_id/input/x_preact/filter"
import { AuthPermissionGrantedFilter } from "../../input/filter/x_preact/input"
import { SearchButton } from "../../../../../common/x_preact/button/search_button"
import { ClearSearchButton } from "../../../../../common/x_preact/button/clear_search_button"

import { SearchAuthUserAccountAction } from "../action"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountForm(props: Props): VNode {
    return box_grow({
        form: true,
        body: container([
            h(LoginIdFilter, { field: props.search.loginId }),
            h(AuthPermissionGrantedFilter, { field: props.search.granted }),
        ]),
        footer: buttons({ left: h(Search, {}), right: h(Clear, {}) }),
    })

    function Search(_props: unknown): VNode {
        const searchState = useApplicationState(props.search.state)
        const observeState = useApplicationState(props.search.observe.state)

        return h(SearchButton, {
            isConnecting: searchState.type === "try",
            observeState,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.search()
        }
    }

    function Clear(_props: unknown): VNode {
        return h(ClearSearchButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.clear()
        }
    }
}
