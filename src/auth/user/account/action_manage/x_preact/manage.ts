import { h, VNode } from "preact"

import { ManageUserAccountResource } from "../resource"

import { SearchAuthUserAccountEntry } from "../../action_search/x_preact/search"

export function ManageUserAccountEntry(resource: ManageUserAccountResource): VNode {
    // TODO resource.edit の state が new か edit なら sidebar レイアウトを使う

    return h(SearchAuthUserAccountEntry, {
        search: resource.search,
    })
}