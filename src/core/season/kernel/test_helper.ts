import { Season, SeasonPeriod } from "./data"

export function markSeason(value: Readonly<{ year: number; period: SeasonPeriod }>): Season {
    return value as Season
}
