import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountResource } from "../resource"

import { SearchAuthUserAccountFormEntry } from "./form"
import { SearchAuthUserAccountPagerEntry } from "./pager"
import { SearchAuthUserAccountColumnsEntry } from "./columns"
import { SearchAuthUserAccountTableEntry } from "./table"

import { useSearchAuthUserAccountTableStructure } from "./structure"

export function SearchAuthUserAccountEntry(resource: SearchAuthUserAccountResource): VNode {
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
