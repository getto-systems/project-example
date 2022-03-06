import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { fieldError } from "../../../../../z_vendor/getto-css/preact/design/form"

import { SearchColumnsEntry } from "../../../../../z_lib/ui/search/columns/x_preact/columns"

import { repositoryErrorReason } from "../../../../../z_lib/ui/repository/x_error/reason"

import { ListAuthUserAccountAction } from "../action"
import { SearchColumnsState } from "../../../../../z_lib/ui/search/columns/action"

import { SearchAuthUserAccountTableStructure } from "./structure"

import { RepositoryError } from "../../../../../z_lib/ui/repository/data"

type EntryProps = Readonly<{
    list: ListAuthUserAccountAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountColumnsEntry(resource: EntryProps): VNode {
    return h(SearchAuthUserAccountColumnsComponent, {
        ...resource,
        state: useApplicationAction(resource.list),
        columnsState: useApplicationAction(resource.list.columns),
    })
}

type Props = EntryProps &
    Readonly<{
        columnsState: SearchColumnsState
    }>
export function SearchAuthUserAccountColumnsComponent(props: Props): VNode {
    return basedOn(props)

    function basedOn({ columnsState }: Props): VNode {
        switch (columnsState.type) {
            case "initial-search":
            case "succeed-to-load":
            case "succeed-to-save":
                return content()

            case "repository-error":
                return errorMessage(columnsState.err)
        }
    }

    function content(): VNode {
        return h(SearchColumnsEntry, {
            field: props.list.columns,
            columns: props.structure.view(),
        })
    }
    function errorMessage(err: RepositoryError): VNode {
        return fieldError(repositoryError(err))
    }
}
function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりカラムの選択に失敗しました`,
        ...reason.detail,
    ])
}
