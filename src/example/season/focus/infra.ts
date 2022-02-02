import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime } from "../../../z_lib/ui/config/infra"

import { SeasonRepository } from "../kernel/infra"

export type FocusSeasonInfra = Readonly<{
    season: SeasonRepository
    clock: Clock
    config: Readonly<{
        focusSeasonExpire: ExpireTime
    }>
}>
