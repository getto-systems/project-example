import { FetchRepositoryResult, StoreRepositoryResult } from "../../../z_lib/ui/repository/infra"
import { Season } from "./data"

export interface SeasonRepository {
    get(): Promise<FetchRepositoryResult<SeasonExpires>>
    set(value: SeasonExpires): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type SeasonExpires = Readonly<{
    season: Season
    expires: number
}>
export type SeasonRepositoryValue = Readonly<{
    season: Readonly<{
        year: number
        period: string
    }>
    expires: number | Long.Long
}>
