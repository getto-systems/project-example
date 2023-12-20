import { h } from "preact"
import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../../x_preact/node"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

import { field, fieldHelp_error, pager } from "../../../../../z_vendor/getto-css/preact/design/form"
import { pagerCount, pagerParams } from "../../../../x_preact/design/table"
import { pagerOptions } from "../../../../../z_vendor/getto-css/preact/design/table"
import { SelectBoard } from "../../../board/input/x_preact/select"
import { LoadButton } from "../../../../x_preact/button/load_button"

import { Atom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../load/data"
import { OffsetFilterBoard } from "../../../board/filter/action"

import { SearchPageResponseResult } from "../../kernel/data"

export function SearchPager<E>(
    props: Readonly<{
        search: {
            readonly page: Atom<LoadState<SearchPageResponseResult<E>>>
            readonly offset: OffsetFilterBoard
            load(): void
        }
        error: (err: E) => readonly PreactContent[]
    }>,
): PreactNode {
    const state = useAtom(props.search.page)

    if (!state.isLoad) {
        return html``
    }
    if (!state.data.isSuccess) {
        return fieldHelp_error(props.error(state.data.err))
    }

    return field({
        title: pagerCount(state.data.page.count),
        body: [
            pager(
                h(SelectBoard, {
                    input: props.search.offset.input,
                    options: pagerOptions(pagerParams(state.data.page)),
                }),
            ),
            button(),
        ],
    })

    function button(): PreactNode {
        return h(LoadButton, { onClick })

        function onClick(e: Event) {
            e.preventDefault()
            props.search.load()
        }
    }
}
