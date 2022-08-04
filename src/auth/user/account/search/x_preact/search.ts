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
export function SearchAuthUserAccount(props: Props): VNode {
    const structure = useAuthUserAccountTableStructure(props.search)

    return html`
        ${container([h(SearchAuthUserAccountForm, props)])}
        ${container([
            box({ body: h(SearchAuthUserAccountPager, props) }),
            box_grow({
                body: h(SearchAuthUserAccountColumns, { structure, search: props.search }),
            }),
        ])}
        ${h(SearchAuthUserAccountTable, { structure, search: props.search })}
    `
}
