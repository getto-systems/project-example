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

import { SearchAuthUserAccountTableStructure } from "./structure"

import { SearchAuthUserAccountResource, SearchAuthUserAccountTableResourceState } from "../resource"

import { SearchColumns } from "../../../../../z_lib/ui/search/columns/data"
import { SearchAuthUserAccountRemoteResponse } from "../../search/data"

type Resource = SearchAuthUserAccountResource & Readonly<{ structure: SearchAuthUserAccountTableStructure }>

export function SearchAuthUserAccountTableEntry(resource: Resource): VNode {
    return h(SearchAuthUserAccountTableComponent, {
        ...resource,
        state: useApplicationAction(resource.search),
        columns: useApplicationAction(resource.search.columns),
    })
}

type Props = Resource & SearchAuthUserAccountTableResourceState
export function SearchAuthUserAccountTableComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ state, columns }: SearchAuthUserAccountTableResourceState): VNode {
        switch (columns.type) {
            case "repository-error":
            case "initial-search":
                return EMPTY_CONTENT

            case "succeed-to-load":
            case "succeed-to-save":
                if (state.type === "succeed-to-search") {
                    return content({ columns: columns.columns, response: state.response })
                }
                return EMPTY_CONTENT
        }
    }

    type Content = Readonly<{ columns: SearchColumns; response: SearchAuthUserAccountRemoteResponse }>

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
