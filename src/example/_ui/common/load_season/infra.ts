import { Clock } from "../../../../z_details/_ui/clock/infra"
import {
    FetchRepositoryResult,
    RepositoryPod,
    StoreRepositoryResult,
} from "../../../../z_details/_ui/repository/infra"

import { Season } from "./data"

export type LoadSeasonInfra = Readonly<{
    season: SeasonRepository
    clock: Clock
}>

export type SeasonRepositoryPod = RepositoryPod<Season, SeasonRepositoryValue>
export interface SeasonRepository {
    get(): Promise<FetchRepositoryResult<Season>>
    set(value: Season): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type SeasonRepositoryValue = Readonly<{
    year: number
}>
