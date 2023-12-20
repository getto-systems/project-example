import { h } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"
import { PreactNode } from "../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import {
    scrollToPosition,
    scrollToFocused,
} from "../../../../../common/util/scroll/x_preact/helper"
import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountPager } from "./pager"
import { SearchAuthUserAccountTable } from "./table"

import { SearchAuthUserAccountAction } from "../action"
import { SearchColumnsBoard } from "../../../../../common/util/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
    columns: SearchColumnsBoard
    structure: SearchAuthUserAccountTableStructure
}>
export function ListAuthUserAccount(props: Props): PreactNode {
    useScrollToFocused(props.search)

    return html`
        ${container([box_grow({ body: h(SearchAuthUserAccountPager, props) })])}
        ${h(SearchAuthUserAccountTable, props)}
    `
}

function useScrollToFocused(search: SearchAuthUserAccountAction): void {
    const state = useAtom(search.focus.scroll)
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
