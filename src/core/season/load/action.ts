import { Atom, initAtom } from "../../../z_vendor/getto-atom/atom"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../common/util/clock/infra"

import { DetectedSeason } from "../kernel/data"
import { RepositoryError } from "../../../common/util/repository/data"

export interface LoadSeasonAction {
    readonly state: Atom<LoadSeasonState>
    load(): Promise<LoadSeasonState>
}

export type LoadSeasonMaterial = Readonly<{
    seasonRepository: SeasonRepository
    clock: Clock
}>

export type LoadSeasonState = Readonly<{ type: "initial" }> | LoadSeasonEvent

const initialState: LoadSeasonState = { type: "initial" }

export function initLoadSeasonAction(material: LoadSeasonMaterial): LoadSeasonAction {
    const { state, post } = initAtom({ initialState, ignite: load })
    return { state, load }

    function load(): Promise<LoadSeasonState> {
        return loadSeason(material, post)
    }
}

type LoadSeasonEvent =
    | Readonly<{ type: "failed"; err: RepositoryError }>
    | Readonly<{ type: "success"; season: DetectedSeason }>

async function loadSeason<S>(
    infra: LoadSeasonMaterial,
    post: Post<LoadSeasonEvent, S>,
): Promise<S> {
    const { clock, seasonRepository } = infra

    const result = await seasonRepository.get()
    if (!result.success) {
        return post({ type: "failed", err: result.err })
    }
    if (!result.found || result.value.expires < clock.now().getTime()) {
        return post({ type: "success", season: { default: true } })
    }
    return post({ type: "success", season: { default: false, season: result.value.season } })
}

interface Post<E, S> {
    (event: E): S
}
