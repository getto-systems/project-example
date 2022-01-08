import { h, VNode } from "preact"
import { useLayoutEffect } from "preact/hooks"

import { useApplicationAction } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import { box_grow } from "../../../../../../ui/vendor/getto-css/preact/design/box"
import { fieldError } from "../../../../../../ui/vendor/getto-css/preact/design/form"

import { SearchColumnsComponent } from "../../../../../z_lib/ui/search/columns/x_preact/columns"

import { repositoryErrorReason } from "../../../../../z_lib/ui/repository/reason"

import { SearchAuthUserAccountAction } from "../action"
import { SearchColumnsState } from "../../../../../z_lib/ui/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

import { RepositoryError } from "../../../../../z_lib/ui/repository/data"

type EntryProps = Readonly<{
    search: SearchAuthUserAccountAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountColumnsEntry(resource: EntryProps): VNode {
    return h(SearchAuthUserAccountColumnsComponent, {
        ...resource,
        state: useApplicationAction(resource.search),
        columns: useApplicationAction(resource.search.columns),
    })
}

type Props = EntryProps &
    Readonly<{
        columns: SearchColumnsState
    }>
export function SearchAuthUserAccountColumnsComponent(props: Props): VNode {
    useLayoutEffect(() => {
        props.search.columns.load(props.structure.initiallyVisibleCells())
    }, [props.search.columns, props.structure])

    return basedOn(props)

    function basedOn({ columns }: Props): VNode {
        switch (columns.type) {
            case "initial-search":
            case "succeed-to-load":
            case "succeed-to-save":
                return columnsBox()

            case "repository-error":
                return errorMessage(columns.err)
        }
    }

    function columnsBox(): VNode {
        return box_grow({
            body: h(SearchColumnsComponent, {
                field: props.search.columns,
                columns: props.structure.view(),
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
