import { h, VNode } from "preact"

import { useApplicationView } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { ManageUserAccountView } from "../resource"

import { SearchUserAccountEntry } from "../../action_search/x_preact/search"

export function ManageUserAccountEntry(view: ManageUserAccountView): VNode {
    const resource = useApplicationView(view)

    // TODO resource.edit の state が new か edit なら sidebar レイアウトを使う

    return h(SearchUserAccountEntry, {
        search: resource.search,
    })
}
