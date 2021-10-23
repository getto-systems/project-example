import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { box_grow } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { SearchColumnsComponent } from "../../../../../z_lib/ui/search/action_columns/x_preact/columns"

import { SearchUserAccountColumnsResourceState, SearchUserAccountResource } from "../resource"

type Props = SearchUserAccountResource & SearchUserAccountColumnsResourceState
export function SearchUserAccountColumnsEntry({ search, label }: Props): VNode {
    return h(SearchUserAccountColumnsComponent, {
        search,
        label,
        columns: useApplicationAction(search.columns),
    })
}

export function SearchUserAccountColumnsComponent({ search, label }: Props): VNode {
    return box_grow({
        body: h(SearchColumnsComponent, {
            field: search.columns,
            label,
        }),
    })
}
