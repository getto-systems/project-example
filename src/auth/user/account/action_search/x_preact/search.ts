import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { SearchUserAccountResource } from "../resource"

import { SearchUserAccountFormEntry } from "./form"
import { SearchUserAccountPagerEntry } from "./pager"
import { SearchUserAccountColumnsEntry } from "./columns"
import { SearchUserAccountTableEntry } from "./table"

import { useSearchUserAccountTableStructure } from "./structure"

export function SearchUserAccountEntry(resource: SearchUserAccountResource): VNode {
    const structure = useSearchUserAccountTableStructure(resource.search)

    return html`
        ${container([h(SearchUserAccountFormEntry, resource)])}
        ${container([
            h(SearchUserAccountPagerEntry, resource),
            h(SearchUserAccountColumnsEntry, { structure, ...resource }),
        ])}
        ${h(SearchUserAccountTableEntry, { structure, ...resource })}
    `
}
