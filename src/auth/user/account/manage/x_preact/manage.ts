import { h, VNode } from "preact"

import { SearchAuthUserAccountEntry } from "../../search/x_preact/search"

import { SearchAuthUserAccountAction } from "../../search/action"

type EntryProps = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function ManageUserAccountEntry(resource: EntryProps): VNode {
    // TODO resource.edit の state が new か edit なら sidebar レイアウトを使う

    return h(SearchAuthUserAccountEntry, {
        search: resource.search,
    })
}
