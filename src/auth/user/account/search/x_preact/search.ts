import { h } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"
import { PreactNode } from "../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { scrollToPosition } from "../../../../../common/util/scroll/x_preact/helper"
import { box, box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchColumns } from "../../../../../common/util/search/columns/x_preact/columns"
import { SearchAuthUserAccountForm } from "./form"
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
export function SearchAuthUserAccount(props: Props): PreactNode {
    useRestoreScrollPosition(props.search)

    return html`
        ${container([h(SearchAuthUserAccountForm, props)])}
        ${container([
            box({ body: h(SearchAuthUserAccountPager, props) }),
            box_grow({ body: h(SearchColumns, { filter: props.columns }) }),
        ])}
        ${h(SearchAuthUserAccountTable, props)}
    `
}

function useRestoreScrollPosition(search: SearchAuthUserAccountAction): void {
    const state = useAtom(search.focus.scroll)
    useEffect(() => {
        switch (state.type) {
            case "close":
                scrollToPosition(document.documentElement, state.position)
                break
        }
    }, [state])
}
