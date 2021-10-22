import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { SearchUserAccountResource } from "../resource"

import { SearchUserAccountFormEntry } from "./form"
import { SearchUserAccountPagerEntry } from "./pager"

export function SearchUserAccountEntry(resource: SearchUserAccountResource): VNode {
    return html`
        ${container([h(SearchUserAccountFormEntry, resource)])}
        ${container([h(SearchUserAccountPagerEntry, resource)])}
    `
}
