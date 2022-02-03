import { Clock } from "../../../z_lib/ui/clock/infra"

import { SeasonRepository } from "../kernel/infra"

export type LoadSeasonInfra = Readonly<{
    season: SeasonRepository
    clock: Clock
}>
