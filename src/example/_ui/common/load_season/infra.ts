import { Clock } from "../../../../z_details/_ui/clock/infra"
import { RepositoryPod } from "../../../../z_details/_ui/repository/infra"

import { Season } from "./data"

export type LoadSeasonInfra = Readonly<{
    season: SeasonRepositoryPod
    clock: Clock
}>

export type SeasonRepositoryPod = RepositoryPod<Season, SeasonRepositoryValue>
export type SeasonRepositoryValue = Readonly<{
    year: number
}>
