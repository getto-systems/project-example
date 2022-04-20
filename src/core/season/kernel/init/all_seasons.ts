import { Season, seasonPeriods } from "../data"
import { beginningOfSystemSeason } from "../../../../x_content/season"

export function allSeasons(currentSeason: Season): readonly Season[] {
    const seasons: Season[] = Array.from(beginningOfSystemSeason)

    for (let year = seasons[0].year + 1; year <= currentSeason.year; year++) {
        if (year < currentSeason.year || currentSeason.period === "winter") {
            seasonPeriods.forEach((period) => {
                seasons.push({ year, period } as Season)
            })
        } else {
            seasons.push({ year, period: "summer" } as Season)
        }
    }

    // 新しいやつが上に来るように reverse する
    return seasons.reverse()
}
