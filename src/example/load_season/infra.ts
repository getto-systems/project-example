import { Clock } from "../../z_lib/ui/clock/infra"
import {
    FetchRepositoryResult,
    StoreRepositoryResult,
} from "../../z_lib/ui/repository/infra"

import { Season } from "./data"

export type LoadSeasonInfra = Readonly<{
    season: SeasonRepository
    clock: Clock
}>

export interface SeasonRepository {
    get(): Promise<FetchRepositoryResult<Season>>
    set(value: Season): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type SeasonRepositoryValue = Readonly<{
    year: number
}>
