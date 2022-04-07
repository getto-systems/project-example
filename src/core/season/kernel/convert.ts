import { RepositoryConverter } from "../../../z_lib/ui/repository/infra"
import { SeasonExpires, SeasonRepositoryValue } from "./infra"

import { ConvertSeasonResult, Season } from "./data"
import { BoardValue } from "../../../z_vendor/getto-application/board/kernel/data"

// TODO Basket をやめる
type SeasonBasket = Readonly<{
    year: number
    period: string
}>
export function seasonToBoardValue(season: SeasonBasket): BoardValue {
    return `${season.year}.${season.period}` as BoardValue
}
export function seasonBoardConverter(
    availableSeasons: readonly Season[],
    value: BoardValue,
): ConvertSeasonResult {
    if (value === "") {
        return { valid: true, default: true }
    }

    const result = findSeason(availableSeasons, value)
    if (!result.found) {
        return { valid: false }
    }
    return { valid: true, default: false, season: result.season }
}

export function seasonRepositoryConverter(
    availableSeasons: readonly Season[],
): RepositoryConverter<SeasonExpires, SeasonRepositoryValue> {
    return {
        toRepository: (value) => value,
        fromRepository: (value) => {
            const result = findSeason(availableSeasons, seasonToBoardValue(value.season))
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
        if (seasonToBoardValue(season) === value) {
            return { found: true, season }
        }
    }
    return { found: false }
}
