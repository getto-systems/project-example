import { Clock } from "../../../../../ui/vendor/getto-application/infra/clock/infra"
import { RepositoryPod } from "../../../../../ui/vendor/getto-application/infra/repository/infra"

import { Season } from "./data"

export type LoadSeasonInfra = Readonly<{
    season: SeasonRepositoryPod
    clock: Clock
}>

export type SeasonRepositoryPod = RepositoryPod<Season, SeasonRepositoryValue>
export type SeasonRepositoryValue = Readonly<{
    year: number
}>
