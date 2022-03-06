import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box, box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountFormEntry } from "./form"
import { SearchAuthUserAccountPagerEntry } from "./pager"
import { SearchAuthUserAccountColumnsEntry } from "./columns"
import { SearchAuthUserAccountTableEntry } from "./table"

import { SearchAuthUserAccountAction } from "../action"

import { useAuthUserAccountTableStructure } from "./structure"

type EntryProps = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountEntry(resource: EntryProps): VNode {
    const structure = useAuthUserAccountTableStructure(resource.search)

    return html`
        ${container([h(SearchAuthUserAccountFormEntry, resource)])}
        ${container([
            box({ body: h(SearchAuthUserAccountPagerEntry, { list: resource.search }) }),
            box_grow({
                body: h(SearchAuthUserAccountColumnsEntry, { structure, list: resource.search }),
            }),
        ])}
        ${h(SearchAuthUserAccountTableEntry, { structure, list: resource.search })}
    `
}
