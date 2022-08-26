import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { scrollToPosition, scrollToFocused } from "../../../../../z_lib/ui/scroll/x_preact/helper"
import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountPager } from "./pager"
import { SearchAuthUserAccountTable } from "./table"

import { SearchAuthUserAccountAction } from "../action"
import { SearchColumnsAction } from "../../../../../z_lib/ui/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
    columns: SearchColumnsAction
    structure: SearchAuthUserAccountTableStructure
}>
export function ListAuthUserAccount(props: Props): VNode {
    useScrollToFocused(props.search)

    return html`
        ${container([box_grow({ body: h(SearchAuthUserAccountPager, props) })])}
        ${h(SearchAuthUserAccountTable, props)}
    `
}

function useScrollToFocused(search: SearchAuthUserAccountAction): void {
    const state = useApplicationState(search.list.scroll.state)
    useEffect(() => {
        switch (state.type) {
            case "detect":
                scrollToFocused({
                    container: document.getElementById("sidebar"),
                    element: document.getElementById("focused"),
                })
                break

            case "focus-change":
                scrollToPosition(document.getElementById("sidebar"), state.position)
                break
        }
    }, [state])
}
