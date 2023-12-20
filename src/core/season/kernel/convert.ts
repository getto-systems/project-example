import { RepositoryConverter } from "../../../common/util/repository/infra"
import { SeasonExpires, SeasonRepositoryValue } from "./infra"

import { Season, DetectedSeason } from "./data"

export function seasonToString(data: DetectedSeason): string {
    if (data.default) {
        return ""
    } else {
        return seasonString(data.season)
    }
}
function seasonString(
    season: Readonly<{
        year: number
        period: string
    }>,
): string {
    return `${season.year}.${season.period}`
}

export function seasonRepositoryConverter(
    availableSeasons: readonly Season[],
): RepositoryConverter<SeasonExpires, SeasonRepositoryValue> {
    return {
        toRepository: (value) => value,
        fromRepository: (value) => {
            const result = findSeason(availableSeasons, seasonString(value.season))
            if (!result.found) {
                return { valid: false }
            }
            return { valid: true, value: value as SeasonExpires }
        },
    }
}

type FindSeasonResult = Readonly<{ found: false }> | Readonly<{ found: true; season: Season }>
function findSeason(availableSeasons: readonly Season[], value: string): FindSeasonResult {
    for (const season of availableSeasons) {
        if (seasonString(season) === value) {
            return { found: true, season }
        }
    }
    return { found: false }
}
