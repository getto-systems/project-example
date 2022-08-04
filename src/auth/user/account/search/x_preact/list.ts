import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"

import { useAuthUserAccountTableStructure } from "./structure"
import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { scrollToFocused } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountPager } from "./pager"
import { SearchAuthUserAccountTable } from "./table"

import { SearchAuthUserAccountAction } from "../action"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function ListAuthUserAccount(props: Props): VNode {
    const structure = useAuthUserAccountTableStructure(props.search)
    useScrollToFocused(props.search)

    return html`
        ${container([box_grow({ body: h(SearchAuthUserAccountPager, props) })])}
        ${h(SearchAuthUserAccountTable, { structure, ...props })}
    `
}

function useScrollToFocused(search: SearchAuthUserAccountAction): void {
    const state = useApplicationAction(search.list.focus)
    useEffect(() => {
        scrollToFocused({
            sidebarId: "sidebar",
            focusedId: "focused",
            isFirstTime: state.type === "detect",
        })
    }, [state])
}
