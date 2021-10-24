import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    table,
    tableColumn,
    tableHeader,
    tbody,
    thead,
} from "../../../../../../ui/vendor/getto-css/preact/design/data"

import { SearchUserAccountTableStructure } from "./structure"

import { SearchUserAccountResource, SearchUserAccountTableResourceState } from "../resource"

import { SearchColumns } from "../../../../../z_lib/ui/search/columns/data"
import { SearchUserAccountRemoteResponse } from "../../search/data"

type Resource = SearchUserAccountResource & Readonly<{ structure: SearchUserAccountTableStructure }>

export function SearchUserAccountTableEntry(resource: Resource): VNode {
    return h(SearchUserAccountTableComponent, {
        ...resource,
        state: useApplicationAction(resource.search),
        columns: useApplicationAction(resource.search.columns),
    })
}

type Props = Resource & SearchUserAccountTableResourceState
export function SearchUserAccountTableComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, columns }: SearchUserAccountTableResourceState): VNode {
        switch (columns.type) {
            case "repository-error":
            case "initial-search":
                return EMPTY_CONTENT

            case "succeed-to-load":
            case "succeed-to-save":
                switch (state.type) {
                    case "initial-search":
                    case "failed-to-search":
                        return EMPTY_CONTENT

                    case "try-to-search":
                    case "take-longtime-to-search":
                        // TODO 前回の結果をそのまま表示したかったけど、どうだろうか
                        return html`前回の結果のままにするか？`

                    case "succeed-to-search":
                        return content({
                            columns: columns.columns,
                            response: state.response,
                        })

                }
        }
    }

    type Content = Readonly<{ columns: SearchColumns; response: SearchUserAccountRemoteResponse }>

    function content({ columns, response }: Content): VNode {
        const params = { summary: response.summary, visibleKeys: columns }

        const sticky = props.structure.sticky()
        const header = props.structure.header(params)

        return table(sticky, [
            thead(tableHeader({ sticky, header })),
            tbody(
                response.users.flatMap((row) =>
                    tableColumn({ sticky, column: props.structure.column(params, row) }),
                ),
            ),
        ])
    }
}

const EMPTY_CONTENT = html``
