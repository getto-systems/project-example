import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { fieldError } from "../../../../../z_vendor/getto-css/preact/design/form"

import { SearchColumns } from "../../../../../z_lib/ui/search/columns/x_preact/columns"

import { repositoryErrorReason } from "../../../../../z_lib/ui/repository/x_error/reason"

import { ListAuthUserAccountAction } from "../action"

import { SearchAuthUserAccountTableStructure } from "./structure"

import { RepositoryError } from "../../../../../z_lib/ui/repository/data"

type Props = Readonly<{
    list: ListAuthUserAccountAction
    structure: SearchAuthUserAccountTableStructure
}>
export function SearchAuthUserAccountColumns(props: Props): VNode {
    const columnsState = useApplicationAction(props.list.columns)
    switch (columnsState.type) {
        case "initial":
        case "success":
            return h(SearchColumns, {
                field: props.list.columns,
                columns: props.structure.view(),
            })

        case "repository-error":
            return fieldError(repositoryError(columnsState.err))
    }
}
function repositoryError(err: RepositoryError): readonly string[] {
    return repositoryErrorReason(err, (reason) => [
        `${reason.message}によりカラムの選択に失敗しました`,
        ...reason.detail,
    ])
}
