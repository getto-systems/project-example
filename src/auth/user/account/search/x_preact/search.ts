import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { scrollToPosition } from "../../../../../common/util/scroll/x_preact/helper"
import { box, box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountForm } from "./form"
import { SearchAuthUserAccountPager } from "./pager"
import { SearchAuthUserAccountColumns } from "./columns"
import { SearchAuthUserAccountTable } from "./table"

import { SearchAuthUserAccountAction } from "../action"
import { SearchColumnsAction } from "../../../../../common/util/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
    columns: SearchColumnsAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccount(props: Props): VNode {
    useRestoreScrollPosition(props.search)

    return html`
        ${container([h(SearchAuthUserAccountForm, props)])}
        ${container([
            box({ body: h(SearchAuthUserAccountPager, props) }),
            box_grow({ body: h(SearchAuthUserAccountColumns, props) }),
        ])}
        ${h(SearchAuthUserAccountTable, props)}
    `
}

function useRestoreScrollPosition(search: SearchAuthUserAccountAction): void {
    const state = useApplicationState(search.list.scroll.state)
    useEffect(() => {
        switch (state.type) {
            case "close":
                scrollToPosition(document.documentElement, state.position)
                break
        }
    }, [state])
}
