import { RepositoryConverter } from "../../../z_lib/ui/repository/infra"

import { Clock } from "../../../z_lib/ui/clock/infra"
import { SeasonExpires, SeasonRepositoryValue } from "./infra"

import { beginningOfSystemSeason, ConvertSeasonResult, Season, seasonPeriods } from "./data"
import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"

export function seasonToBoardValue(season: Season): BoardValue {
    return `${season.year}.${season.period}` as BoardValue
}
export function seasonBoardConverter(value: BoardValue): ConvertSeasonResult {
    if (value === "") {
        return { valid: true, default: true }
    }

    const pair = value.split(".")
    if (pair.length !== 2) {
        return { valid: false }
    }

    const year = parseInt(pair[0])
    const period = pair[1]

    if (!isValidYear(year)) {
        return { valid: false }
    }
    if (!isValidPeriod(period)) {
        return { valid: false }
    }
    return { valid: true, default: false, season: markSeason({ year, period }) }
}

export const seasonRepositoryConverter: RepositoryConverter<SeasonExpires, SeasonRepositoryValue> =
    {
        toRepository: (value) => value,
        fromRepository: (value) => {
            if (!isValidYear(value.season.year)) {
                return { valid: false }
            }
            if (!isValidPeriod(value.season.period)) {
                return { valid: false }
            }
            return { valid: true, value: markSeasonExpires(value) }
        },
    }

function isValidYear(year: number): boolean {
    return year >= beginningOfSystemSeason.year
}
function isValidPeriod(period: string): boolean {
    // seasonPeriods は string の配列にしないと includes で判定できない
    return seasonPeriods.map((period) => `${period}`).includes(period)
}

export function defaultSeason(clock: Clock): Season {
    const now = clock.now()
    const year = now.getFullYear()
    const month = now.getMonth()

    if (month < 3) {
        // 1, 2, 3月は前の年の winter
        return markSeason({ year: year - 1, period: "winter" })
    }
    if (month > 8) {
        // 10, 11, 12月はその年の winter
        return markSeason({ year, period: "winter" })
    }
    // 4, 5, 6, 7, 8, 9月はその年の summer
    return markSeason({ year, period: "summer" })
}

function markSeason(season: Readonly<{ year: number; period: string }>): Season {
    return season as Season
}
function markSeasonExpires(season: SeasonRepositoryValue): SeasonExpires {
    return season as SeasonExpires
}
