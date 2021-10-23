import { h, VNode } from "preact"
import { html } from "htm/preact"

import { container } from "../../../../../../ui/vendor/getto-css/preact/design/box"

import { SearchUserAccountResource } from "../resource"

import { SearchUserAccountFormEntry } from "./form"
import { SearchUserAccountPagerEntry } from "./pager"
import { SearchUserAccountColumnsEntry } from "./columns"

export function SearchUserAccountEntry(resource: SearchUserAccountResource): VNode {
    return html`
        ${container([h(SearchUserAccountFormEntry, resource)])}
        ${container([
            h(SearchUserAccountPagerEntry, resource),
            // TODO label は table の structure からヘッダの内容を取得するようにする
            h(SearchUserAccountColumnsEntry, { ...resource, label: (key) => `カラム-${key}` }),
        ])}
    `
}
