import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../../ui/vendor/getto-application/action/action"

import { defaultSeason } from "../kernel/convert"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"

import { beginningOfSystemSeason, Season, seasonPeriods } from "../kernel/data"
import { RepositoryError } from "../../../z_lib/ui/repository/data"

export type LoadSeasonAction = StatefulApplicationAction<LoadSeasonState>

export type LoadSeasonMaterial = Readonly<{
    season: SeasonRepository
    clock: Clock
}>

export type LoadSeasonState = Readonly<{ type: "initial-season" }> | LoadSeasonEvent

export const initialLoadSeasonState: LoadSeasonState = { type: "initial-season" }

export function initLoadSeasonAction(material: LoadSeasonMaterial): LoadSeasonAction {
    return new Action(material)
}

class Action extends AbstractStatefulApplicationAction<LoadSeasonState> {
    readonly initialState = initialLoadSeasonState

    constructor(material: LoadSeasonMaterial) {
        super({ ignite: () => loadSeason(material, this.post) })
    }
}

type LoadSeasonEvent =
    | Readonly<{
          type: "succeed-to-load"
          season: Season
          default: boolean
          availableSeasons: Season[]
      }>
    | Readonly<{ type: "failed-to-load"; err: RepositoryError }>

async function loadSeason<S>(
    infra: LoadSeasonMaterial,
    post: Post<LoadSeasonEvent, S>,
): Promise<S> {
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
