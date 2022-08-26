import { RepositoryOutsideFeature } from "../../../repository/feature"

import { newSearchColumnsRepository } from "./columns_repository"

import { initSearchColumnsAction, SearchColumnsAction } from "../action"

export function newSearchColumnsAction(
    feature: RepositoryOutsideFeature,
    key: string,
    initiallyVisibleCells: readonly string[],
): SearchColumnsAction {
    return initSearchColumnsAction(
        {
            columnsRepository: newSearchColumnsRepository(feature, key),
        },
        initiallyVisibleCells,
    )
}
