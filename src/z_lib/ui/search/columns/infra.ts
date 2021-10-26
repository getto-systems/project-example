import { FetchRepositoryResult, StoreRepositoryResult } from "../../repository/infra"

import { SearchColumns } from "./data";

export type SearchColumnsInfra = Readonly<{
    columns: SearchColumnsRepository
}>

export interface SearchColumnsRepository {
    get(): Promise<FetchRepositoryResult<SearchColumns>>
    set(value: SearchColumns): Promise<StoreRepositoryResult>
}
export type SearchColumnsRepositoryValue = readonly string[]
