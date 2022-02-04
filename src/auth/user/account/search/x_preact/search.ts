import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountFormEntry } from "./form"
import { SearchAuthUserAccountPagerEntry } from "./pager"
import { SearchAuthUserAccountColumnsEntry } from "./columns"
import { SearchAuthUserAccountTableEntry } from "./table"

import { SearchAuthUserAccountAction } from "../action"

import { useSearchAuthUserAccountTableStructure } from "./structure"

type EntryProps = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountEntry(resource: EntryProps): VNode {
    const structure = useSearchAuthUserAccountTableStructure(resource.search)

    return html`
        ${container([h(SearchAuthUserAccountFormEntry, resource)])}
        ${container([
            h(SearchAuthUserAccountPagerEntry, resource),
            h(SearchAuthUserAccountColumnsEntry, { structure, ...resource }),
        ])}
        ${h(SearchAuthUserAccountTableEntry, { structure, ...resource })}
    `
}
