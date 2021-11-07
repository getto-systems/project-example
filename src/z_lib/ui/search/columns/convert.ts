import { RepositoryConverter } from "../../repository/infra"
import { SearchColumnsRepositoryValue } from "./infra"

import { SearchColumns } from "./data"

export const searchColumnsRepositoryConverter: RepositoryConverter<
    SearchColumns,
    SearchColumnsRepositoryValue
> = {
    toRepository: (value) => value,
    fromRepository: (value) => {
        return {
            valid: true,
            value: toSearchColumns(value),
        }
    },
}

export function toSearchColumns(columns: readonly string[]): SearchColumns {
    // string[] と SearchColumns は同じもの
    return columns as SearchColumns
}
