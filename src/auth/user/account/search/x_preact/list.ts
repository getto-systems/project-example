import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountPagerEntry } from "./pager"
import { SearchAuthUserAccountTableEntry } from "./table"

import { ListAuthUserAccountAction } from "../action"

import { useAuthUserAccountTableStructure } from "./structure"

type EntryProps = Readonly<{
    list: ListAuthUserAccountAction
}>
export function ListAuthUserAccountEntry(resource: EntryProps): VNode {
    const structure = useAuthUserAccountTableStructure(resource.list)

    return html`
        ${container([box_grow({ body: h(SearchAuthUserAccountPagerEntry, resource) })])}
        ${h(SearchAuthUserAccountTableEntry, { structure, ...resource })}
    `
}
