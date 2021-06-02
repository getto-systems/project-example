import { RepositoryConverter } from "../../../../../../ui/vendor/getto-application/infra/repository/infra"

import { Clock } from "../../../../../../ui/vendor/getto-application/infra/clock/infra"
import { SeasonRepositoryValue } from "../infra"

import { Season } from "../data"

export const seasonRepositoryConverter: RepositoryConverter<Season, SeasonRepositoryValue> = {
    toRepository: (value) => value,
    fromRepository: (value) => {
        // 2000年以前は無効 : サービス開始時点のつもり
        if (value.year < 2000) {
            return { valid: false }
        }
        return { valid: true, value: markSeason(value) }
    },
}

export function defaultSeason(clock: Clock): Season {
    return markSeason({ year: clock.now().getFullYear() })
}

function markSeason(season: SeasonRepositoryValue): Season {
    return season as Season
}
