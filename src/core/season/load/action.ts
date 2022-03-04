import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../z_vendor/getto-application/action/action"

import { availableSeasons } from "../kernel/init/available_seasons"
import { defaultSeason } from "../kernel/init/default_season"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"

import { Season } from "../kernel/data"
import { RepositoryError } from "../../../z_lib/ui/repository/data"

export interface LoadSeasonAction extends StatefulApplicationAction<LoadSeasonState> {
    load(): Promise<LoadSeasonState>
}

export type LoadSeasonMaterial = Readonly<{
    season: SeasonRepository
    clock: Clock
}>

export type LoadSeasonState = Readonly<{ type: "initial-season" }> | LoadSeasonEvent

const initialState: LoadSeasonState = { type: "initial-season" }

export function initLoadSeasonAction(material: LoadSeasonMaterial): LoadSeasonAction {
    return new Action(material)
}

class Action extends AbstractStatefulApplicationAction<LoadSeasonState> {
    readonly initialState = initialState

    material: LoadSeasonMaterial

    constructor(material: LoadSeasonMaterial) {
        super({ ignite: () => this.load() })

        this.material = material
    }

    load(): Promise<LoadSeasonState> {
        return loadSeason(this.material, this.post)
    }
}

type LoadSeasonEvent =
    | Readonly<{
          type: "succeed-to-load"
          season: Season
          default: boolean
          availableSeasons: readonly Season[]
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

interface Post<E, S> {
    (event: E): S
}
