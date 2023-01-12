import { RepositoryConverter } from "../../../common/util/repository/infra"
import { SeasonExpires, SeasonRepositoryValue } from "./infra"

import { ValidateSeasonResult, Season } from "./data"

export function seasonToString(
    season: Readonly<{
        year: number
        period: string
    }>,
): string {
    return `${season.year}.${season.period}`
}
export function seasonConverter(
    availableSeasons: readonly Season[],
    value: string,
): ValidateSeasonResult {
    if (value === "") {
        return { valid: true, value: { default: true } }
    }

    const result = findSeason(availableSeasons, value)
    if (!result.found) {
        return { valid: false, err: { type: "invalid-season" } }
    }
    return { valid: true, value: { default: false, season: result.season } }
}

export function seasonRepositoryConverter(
    availableSeasons: readonly Season[],
): RepositoryConverter<SeasonExpires, SeasonRepositoryValue> {
    return {
        toRepository: (value) => value,
        fromRepository: (value) => {
            const result = findSeason(availableSeasons, seasonToString(value.season))
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
        if (seasonToString(season) === value) {
            return { found: true, season }
        }
    }
    return { found: false }
}
