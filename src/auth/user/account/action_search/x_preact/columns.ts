import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { box_grow } from "../../../../../../ui/vendor/getto-css/preact/design/box"
import { fieldError } from "../../../../../../ui/vendor/getto-css/preact/design/form"

import { SearchColumnsComponent } from "../../../../../z_lib/ui/search/action_columns/x_preact/columns"

import { repositoryErrorReason } from "../../../../../z_lib/ui/repository/reason"

import { SearchUserAccountTableStructure } from "./structure"

import { SearchUserAccountColumnsResourceState, SearchUserAccountResource } from "../resource"

import { RepositoryError } from "../../../../../z_lib/ui/repository/data"
import { SearchUserAccountRemoteResponse } from "../../search/data"

type Resource = SearchUserAccountResource & Readonly<{ structure: SearchUserAccountTableStructure }>

export function SearchUserAccountColumnsEntry(resource: Resource): VNode {
    return h(SearchUserAccountColumnsComponent, {
        ...resource,
        state: useApplicationAction(resource.search),
        columns: useApplicationAction(resource.search.columns),
    })
}

type Props = Resource & SearchUserAccountColumnsResourceState
export function SearchUserAccountColumnsComponent(props: Props): VNode {
    useLayoutEffect(() => {
        switch (props.state.type) {
            case "succeed-to-search":
                props.search.columns.load(initialColumns(props.state.response))
        }

        function initialColumns(response: SearchUserAccountRemoteResponse): string[] {
            return props.structure
                .view({ summary: response.summary })
                .filter((column) => column.isInitiallyVisible && !column.isAlwaysVisible)
                .map((column) => `${column.key}`)
        }
    }, [props.search.columns, props.structure, props.state])

    return basedOn(props)

    function basedOn({ state, columns }: SearchUserAccountColumnsResourceState): VNode {
        switch (columns.type) {
            case "initial-search":
                return EMPTY_CONTENT

            case "repository-error":
                return errorMessage(columns.err)

            case "succeed-to-load":
            case "succeed-to-save":
                switch (state.type) {
                    case "succeed-to-search":
                        return columnsBox({ response: state.response })

                    default:
                        return EMPTY_CONTENT
                }
        }
    }

    type Content = Readonly<{ response: SearchUserAccountRemoteResponse }>

    function columnsBox({ response }: Content): VNode {
        return box_grow({
            body: h(SearchColumnsComponent, {
                field: props.search.columns,
                columns: props.structure.view({ summary: response.summary }),
            }),
        })
    }
    function errorMessage(err: RepositoryError): VNode {
        return box_grow({ body: fieldError(repositoryError(err)) })
    }
}
function repositoryError(err: RepositoryError): string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりカラムの選択に失敗しました`,
        ...reason.detail,
    ])
}

const EMPTY_CONTENT = box_grow({ body: "" })
