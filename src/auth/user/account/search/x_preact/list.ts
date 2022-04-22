import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"

import { useAuthUserAccountTableStructure } from "./structure"
import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { scrollToFocused } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountPager } from "./pager"
import { SearchAuthUserAccountTable } from "./table"

import { FocusedAuthUserAccountAction, ListAuthUserAccountAction } from "../action"

type Props = Readonly<{
    list: ListAuthUserAccountAction
}>
export function ListAuthUserAccount(resource: Props): VNode {
    const structure = useAuthUserAccountTableStructure(resource.list)
    useScrollToFocused(resource.list.focused)

    return html`
        ${container([box_grow({ body: h(SearchAuthUserAccountPager, resource) })])}
        ${h(SearchAuthUserAccountTable, { structure, ...resource })}
    `
}

function useScrollToFocused(detail: FocusedAuthUserAccountAction): void {
    const state = useApplicationAction(detail)
    useEffect(() => {
        scrollToFocused({
            sidebarId: "sidebar",
            focusedId: "focused",
            isFirstTime: state.type === "focus-detected",
        })
    }, [state])
}
