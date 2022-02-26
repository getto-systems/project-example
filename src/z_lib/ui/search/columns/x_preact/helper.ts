import { SearchColumnsState } from "../action"

import { SearchColumns } from "../data"

export type SearchColumnsResult =
    | Readonly<{ found: false }>
    | Readonly<{ found: true; columns: SearchColumns }>
export function searchColumns(columnsState: SearchColumnsState): SearchColumnsResult {
    switch (columnsState.type) {
        case "repository-error":
        case "initial-search":
            return { found: false }

        case "succeed-to-load":
        case "succeed-to-save":
            return { found: true, columns: columnsState.columns }
    }
}
