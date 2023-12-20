import { RepositoryOutsideFeature } from "../../../repository/feature"

import { newSearchColumnsRepository } from "./columns_repository"

import { initSearchColumnsBoard, SearchColumnsBoard } from "../action"

import { TableDataCell } from "../../../../../z_vendor/getto-table/preact/core"

export function newSearchColumnsBoard(
    feature: RepositoryOutsideFeature,
    key: string,
    cells: readonly TableDataCell[],
): SearchColumnsBoard {
    return initSearchColumnsBoard(
        {
            columnsRepository: newSearchColumnsRepository(feature, key),
        },
        cells,
    )
}
