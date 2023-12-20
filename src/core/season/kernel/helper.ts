import { DetectedSeason, SeasonPeriod } from "./data"

export function seasonLabel(data: DetectedSeason): string {
    if (data.default) {
        return "今シーズン"
    } else {
        return `${data.season.year} ${periodLabel(data.season.period)}`
    }
}
function periodLabel(period: SeasonPeriod): string {
    switch (period) {
        case "summer":
            return "夏"

        case "winter":
            return "冬"
    }
}
