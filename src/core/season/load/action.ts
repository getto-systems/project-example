import {
    initApplicationState,
    StatefulApplicationAction,
} from "../../../z_vendor/getto-application/action/action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"

import { Season } from "../kernel/data"
import { RepositoryError } from "../../../z_lib/ui/repository/data"

export interface LoadSeasonAction extends StatefulApplicationAction<LoadSeasonState> {
    load(): Promise<LoadSeasonState>
}

export type LoadSeasonMaterial = Readonly<{
    defaultSeason: Season
    availableSeasons: readonly Season[]
    seasonRepository: SeasonRepository
    clock: Clock
}>

export type LoadSeasonState = Readonly<{ type: "initial" }> | LoadSeasonEvent

const initialState: LoadSeasonState = { type: "initial" }

export function initLoadSeasonAction(material: LoadSeasonMaterial): LoadSeasonAction {
    const { state, post } = initApplicationState({
        initialState,
        ignite: () => load(),
    })
    return { state, load }

    function load(): Promise<LoadSeasonState> {
        return loadSeason(material, post)
    }
}

type LoadSeasonEvent =
    | Readonly<{ type: "failed"; err: RepositoryError }>
    | Readonly<{
          type: "success"
          season: Season
          default: boolean
          availableSeasons: readonly Season[]
      }>

async function loadSeason<S>(
    infra: LoadSeasonMaterial,
    post: Post<LoadSeasonEvent, S>,
): Promise<S> {
    const { clock, seasonRepository, defaultSeason, availableSeasons } = infra

    const result = await seasonRepository.get()
    if (!result.success) {
        return post({ type: "failed", err: result.err })
    }
    if (!result.found || result.value.expires < clock.now().getTime()) {
        return post({ type: "success", season: defaultSeason, default: true, availableSeasons })
    }
    return post({ type: "success", season: result.value.season, default: false, availableSeasons })
}

interface Post<E, S> {
    (event: E): S
}
