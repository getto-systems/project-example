import { h, VNode } from "preact"
import { useEffect } from "preact/hooks"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountPagerEntry } from "./pager"
import { SearchAuthUserAccountTableEntry } from "./table"

import { DetailAuthUserAccountAction, ListAuthUserAccountAction } from "../action"

import { useAuthUserAccountTableStructure } from "./structure"
import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { scrollToFocused } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

type EntryProps = Readonly<{
    list: ListAuthUserAccountAction
}>
export function ListAuthUserAccountEntry(resource: EntryProps): VNode {
    const structure = useAuthUserAccountTableStructure(resource.list)
    useScrollToFocused(resource.list.detail)

    return html`
        ${container([box_grow({ body: h(SearchAuthUserAccountPagerEntry, resource) })])}
        ${h(SearchAuthUserAccountTableEntry, { structure, ...resource })}
    `
}

function useScrollToFocused(detail: DetailAuthUserAccountAction): void {
    const state = useApplicationAction(detail)
    useEffect(() => {
        scrollToFocused({
            sidebarId: "sidebar",
            focusedId: "focused",
            isFirstTime: state.type === "focus-detected",
        })
    }, [state])
}
