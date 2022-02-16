import { defaultSeason } from "./default_season"

import { Clock } from "../../../../z_lib/ui/clock/infra"

import { Season, seasonPeriods } from "../data"
import { beginningOfSeason } from "../../../../x_content/season"

export function availableSeasons(clock: Clock): readonly Season[] {
    const seasons: Season[] = Array.from(beginningOfSeason)
    const currentSeason = defaultSeason(clock)

    for (let year = seasons[0].year + 1; year <= currentSeason.year; year++) {
        if (year < currentSeason.year || currentSeason.period === "winter") {
            seasonPeriods.forEach((period) => {
                seasons.push({ year, period } as Season)
            })
        } else {
            seasons.push({ year, period: "summer" } as Season)
        }
    }

    // 新しいやつが上に来るようにしたい
    return seasons.reverse()
}
