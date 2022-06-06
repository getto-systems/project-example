import { ValidateBoardFieldResult } from "../../../z_vendor/getto-application/board/validate_field/data"

export type Season = Season_data & { Season: never }
type Season_data = Readonly<{
    year: number
    period: SeasonPeriod
}>

export const seasonPeriods = ["summer", "winter"] as const
export type SeasonPeriod = typeof seasonPeriods[number]

export type ValidateSeasonResult = ValidateBoardFieldResult<DetectedSeason, ValidateSeasonError>

export type DetectedSeason =
    | Readonly<{ default: true }>
    | Readonly<{ default: false; season: Season }>

export type ValidateSeasonError = Readonly<{ type: "invalid-season" }>
