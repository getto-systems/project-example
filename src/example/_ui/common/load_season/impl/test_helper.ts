import { SeasonRepositoryValue } from "../infra"

import { Season } from "../data"

export function markSeason(season: SeasonRepositoryValue): Season {
    return season as Season
}
