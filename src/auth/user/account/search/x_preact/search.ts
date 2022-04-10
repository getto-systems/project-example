import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box, box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { SearchAuthUserAccountForm } from "./form"
import { SearchAuthUserAccountPager } from "./pager"
import { SearchAuthUserAccountColumns } from "./columns"
import { SearchAuthUserAccountTable } from "./table"

import { SearchAuthUserAccountAction } from "../action"

import { useAuthUserAccountTableStructure } from "./structure"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccount(resource: Props): VNode {
    const structure = useAuthUserAccountTableStructure(resource.search)

    return html`
        ${container([h(SearchAuthUserAccountForm, resource)])}
        ${container([
            box({ body: h(SearchAuthUserAccountPager, { list: resource.search }) }),
            box_grow({
                body: h(SearchAuthUserAccountColumns, { structure, list: resource.search }),
            }),
        ])}
        ${h(SearchAuthUserAccountTable, { structure, list: resource.search })}
    `
}
