import { FetchRepositoryResult, StoreRepositoryResult } from "../../repository/infra"

export interface SearchColumnsRepository {
    get(): Promise<FetchRepositoryResult<readonly string[]>>
    set(value: readonly string[]): Promise<StoreRepositoryResult>
}
export type SearchColumnsRepositoryValue = readonly string[]
