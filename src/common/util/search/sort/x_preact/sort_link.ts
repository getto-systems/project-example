import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../x_preact/vnode"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"
import { SortKey, sortSign } from "../../../../../z_vendor/getto-css/preact/design/table"
import { SORT_SIGN } from "../../../../x_preact/design/table"

import { Atom } from "../../../../../z_vendor/getto-atom/atom"

import { SearchSort } from "../data"

export function sortLinkDecorator<K extends SortKey>(
    search: {
        readonly sortKey: Atom<SearchSort<K>>
        sort(key: K): void
    },
    key: K,
): (content: PreactContent) => PreactContent {
    return (content) => h(Sort, { content })

    function Sort(props: Readonly<{ content: PreactContent }>): PreactNode {
        const state = useAtom(search.sortKey)

        return html`<a href="#" onClick=${onClick}>${props.content} ${sign()}</a>`

        function sign() {
            return sortSign(SORT_SIGN, state, key)
        }
        function onClick(e: Event) {
            e.preventDefault()
            search.sort(key)
        }
    }
}
