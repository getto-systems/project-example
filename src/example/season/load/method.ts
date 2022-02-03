import { LoadSeasonInfra } from "./infra"

import { LoadSeasonEvent } from "./event"

import { defaultSeason } from "../kernel/convert"
import { beginningOfSystemSeason, Season, seasonPeriods } from "../kernel/data"
import { Clock } from "../../../z_lib/ui/clock/infra"

export interface LoadSeasonMethod {
    <S>(post: Post<LoadSeasonEvent, S>): Promise<S>
}

interface Load {
    (infra: LoadSeasonInfra): LoadSeasonMethod
}
export const loadSeason: Load = (infra) => async (post) => {
    const { clock, season } = infra

    const result = await season.get()
    if (!result.success) {
        return post({ type: "failed-to-load", err: result.err })
    }
    if (!result.found) {
        return post({
            type: "succeed-to-load",
            season: defaultSeason(clock),
            default: true,
            availableSeasons: availableSeasons(clock),
        })
    }
    if (result.value.expires < clock.now().getTime()) {
        return post({
            type: "succeed-to-load",
            season: defaultSeason(clock),
            default: true,
            availableSeasons: availableSeasons(clock),
        })
    }

    return post({
        type: "succeed-to-load",
        season: result.value.season,
        default: false,
        availableSeasons: availableSeasons(clock),
    })
}
function availableSeasons(clock: Clock): Season[] {
    const seasons: Season[] = []

    const currentYear = clock.now().getFullYear()

    for (let year = beginningOfSystemSeason.year; year < currentYear; year++) {
        seasonPeriods.forEach((period) => {
            seasons.push({ year, period } as Season)
        })
    }

    return seasons
}

interface Post<E, S> {
    (event: E): S
}
