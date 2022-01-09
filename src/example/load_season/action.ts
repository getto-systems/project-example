import { ApplicationStateAction } from "../../../ui/vendor/getto-application/action/action"

import { SeasonRepository } from "./infra"
import { Clock } from "../../z_lib/ui/clock/infra"

import { RepositoryError } from "../../z_lib/ui/repository/data"
import { Season } from "./data"

import { defaultSeason } from "./convert"
import { ApplicationAbstractStateAction } from "../../../ui/vendor/getto-application/action/init"

export type LoadSeasonAction = ApplicationStateAction<LoadSeasonState>

export type LoadSeasonState = Readonly<{ type: "initial-season" }> | LoadSeasonEvent

export const initialLoadSeasonState: LoadSeasonState = { type: "initial-season" }

export type LoadSeasonInfra = Readonly<{
    seasonRepository: SeasonRepository
    clock: Clock
}>

export function initLoadSeasonAction(infra: LoadSeasonInfra): LoadSeasonAction {
    return new Action(infra)
}

class Action extends ApplicationAbstractStateAction<LoadSeasonState> {
    readonly initialState = initialLoadSeasonState

    constructor(infra: LoadSeasonInfra) {
        super(() => loadSeason(infra, this.post))
    }
}

type LoadSeasonEvent =
    | Readonly<{ type: "succeed-to-load"; value: Season }>
    | Readonly<{ type: "failed-to-load"; err: RepositoryError }>

async function loadSeason<S>(infra: LoadSeasonInfra, post: Post<LoadSeasonEvent, S>): Promise<S> {
    const { clock, seasonRepository: season } = infra

    const result = await season.get()
    if (!result.success) {
        return post({ type: "failed-to-load", err: result.err })
    }
    if (!result.found) {
        return post({ type: "succeed-to-load", value: defaultSeason(clock) })
    }
    return post({ type: "succeed-to-load", value: result.value })
}

interface Post<E, S> {
    (event: E): S
}
