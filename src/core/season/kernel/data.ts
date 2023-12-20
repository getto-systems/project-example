import { ValidateBoardValue } from "../../../common/util/board/validate/data"

export type Season = Season_data & { Season: never }
type Season_data = Readonly<{
    year: number
    period: SeasonPeriod
}>

export const seasonPeriods = ["summer", "winter"] as const
export type SeasonPeriod = (typeof seasonPeriods)[number]

export type ValidateSeasonResult = ValidateBoardValue<DetectedSeason, ValidateSeasonError>

export type DetectedSeason =
    | Readonly<{ default: true }>
    | Readonly<{ default: false; season: Season }>

export function defaultSeason(): DetectedSeason {
    return { default: true }
}
export function detectedSeason(season: Season): DetectedSeason {
    return { default: false, season }
}

export type ValidateSeasonError = Readonly<{ type: "invalid-season" }>
