import { Season, SeasonPeriod } from "./data"

export function seasonLabel(season: Season): string {
    return `${season.year} ${periodLabel(season.period)}`
}
function periodLabel(period: SeasonPeriod): string {
    switch (period) {
        case "summer":
            return "夏"

        case "winter":
            return "冬"
    }
}
