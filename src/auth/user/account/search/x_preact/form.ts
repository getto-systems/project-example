import { h } from "preact"
import { PreactNode } from "../../../../../common/x_preact/node"

import { buttons } from "../../../../../z_vendor/getto-css/preact/design/form"
import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { AuthUserLoginIdFilter } from "../../../login_id/input/filter/x_preact/input"
import { AuthPermissionGrantedFilter } from "../../input/filter/x_preact/input"
import { SearchButton } from "../../../../../common/x_preact/button/search_button"
import { ClearSearchButton } from "../../../../../common/x_preact/button/clear_search_button"

import { SearchAuthUserAccountAction } from "../action"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountForm(props: Props): PreactNode {
    return box_grow({
        form: true,
        body: container([
            h(AuthUserLoginIdFilter, { filter: props.search.loginId }),
            h(AuthPermissionGrantedFilter, { filter: props.search.granted }),
        ]),
        footer: buttons({ left: h(Search, {}), right: h(Reset, {}) }),
    })

    function Search(_props: unknown): PreactNode {
        return h(SearchButton, {
            connect: props.search.connect,
            observe: props.search.observe,
            onClick,
        })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.search()
        }
    }

    function Reset(_props: unknown): PreactNode {
        return h(ClearSearchButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.reset()
        }
    }
}
