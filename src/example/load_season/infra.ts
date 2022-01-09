import { FetchRepositoryResult, StoreRepositoryResult } from "../../z_lib/ui/repository/infra"

import { Season } from "./data"

export interface SeasonRepository {
    get(): Promise<FetchRepositoryResult<Season>>
    set(value: Season): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type SeasonRepositoryValue = Readonly<{
    year: number
}>
