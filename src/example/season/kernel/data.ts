export type Season = Season_data & { Season: never }
type Season_data = Readonly<{
    year: number
    period: SeasonPeriod
}>

export const seasonPeriods = ["summer", "winter"] as const
export type SeasonPeriod = typeof seasonPeriods[number]

export const beginningOfSystemSeason = { year: 2021, period: "winter" } as Season

export type ConvertSeasonResult =
    | Readonly<{ valid: false }>
    | Readonly<{ valid: true; default: true }>
    | Readonly<{ valid: true; default: false; season: Season }>
