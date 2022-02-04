import { defaultSeason } from "./default_season"

import { Clock } from "../../../../z_lib/ui/clock/infra"

import { Season, seasonPeriods } from "../data"

export function availableSeasons(clock: Clock): readonly Season[] {
    // サービス開始時点のつもり
    const seasons: Season[] = [
        { year: 2021, period: "summer" } as Season, // winter 始まりだったら summer を除く
        { year: 2021, period: "winter" } as Season, // 動的にやろうとするとめんどくさいのでこうする
    ]

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
