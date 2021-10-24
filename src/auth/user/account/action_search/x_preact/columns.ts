import { h, VNode } from "preact"

import { box_grow } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import {
    SearchColumnsComponent,
    SearchColumnState,
} from "../../../../../z_lib/ui/search/action_columns/x_preact/columns"

import { SearchUserAccountResource } from "../resource"

type Props = SearchUserAccountResource &
    Readonly<{
        columns: SearchColumnState[]
    }>
export function SearchUserAccountColumnsComponent({ search, columns }: Props): VNode {
    return box_grow({
        body: h(SearchColumnsComponent, {
            field: search.columns,
            columns,
        }),
    })
}
