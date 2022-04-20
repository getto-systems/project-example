import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../z_vendor/getto-application/action/action"
import { initInputSeasonAction, InputSeasonAction } from "../input/action"
import { LoadSeasonState } from "../load/action"

import { seasonBoardConverter } from "../kernel/convert"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime } from "../../../z_lib/ui/config/infra"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { Season } from "../kernel/data"

export interface SetupSeasonAction extends StatefulApplicationAction<SetupSeasonState> {
    readonly season: InputSeasonAction

    open(): Promise<SetupSeasonState>
    setup(): Promise<SetupSeasonState>
}

export type SetupSeasonMaterial = Readonly<{
    infra: SetupSeasonInfra
    config: SetupSeasonConfig
}>
export type SetupSeasonInfra = Readonly<{
    availableSeasons: readonly Season[]
    seasonRepository: SeasonRepository
    clock: Clock
}>
export type SetupSeasonConfig = Readonly<{
    manualSetupSeasonExpire: ExpireTime
}>

export type SetupSeasonState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "edit-season" }> // TODO editable にする
    | SetupSeasonEvent

const initialState: SetupSeasonState = { type: "initial" }

export function initSetupSeasonAction(
    material: SetupSeasonMaterial,
    load: LoadAction,
): SetupSeasonAction {
    return new Action(material, load)
}

interface LoadAction {
    ignitionState: Promise<LoadSeasonState>
    load(): Promise<LoadSeasonState>
}

class Action extends AbstractStatefulApplicationAction<SetupSeasonState> {
    readonly initialState = initialState

    readonly season: InputSeasonAction

    material: SetupSeasonMaterial
    load: LoadAction

    field: { (): string }

    constructor(material: SetupSeasonMaterial, load: LoadAction) {
        super()

        const season = initInputSeasonAction()

        load.ignitionState.then((state) => {
            switch (state.type) {
                case "success":
                    season.set(state.season)
            }
        })

        this.season = season.input

        this.material = material
        this.load = load
        this.field = () => season.get()
    }

    setup(): Promise<SetupSeasonState> {
        return setupSeason(this.material, this.field(), (state) => {
            if (state.type === "success") {
                this.load.load()
            }
            return this.post(state)
        })
    }
    async open(): Promise<SetupSeasonState> {
        return this.post({ type: "edit-season" })
    }
}

type SetupSeasonEvent =
    | Readonly<{ type: "invalid" }>
    | Readonly<{ type: "failed"; err: RepositoryError }>
    | Readonly<{ type: "success" }>

async function setupSeason<S>(
    { infra, config }: SetupSeasonMaterial,
    value: string,
    post: Post<SetupSeasonEvent, S>,
): Promise<S> {
    const { clock, seasonRepository, availableSeasons } = infra

    const convertResult = seasonBoardConverter(availableSeasons, value)
    if (!convertResult.valid) {
        return post({ type: "invalid" })
    }

    if (convertResult.default) {
        const result = await seasonRepository.remove()
        if (!result.success) {
            return post({ type: "failed", err: result.err })
        }
        return post({ type: "success" })
    }

    const result = await seasonRepository.set({
        season: convertResult.season,
        expires: clock.now().getTime() + config.manualSetupSeasonExpire.expire_millisecond,
    })
    if (!result.success) {
        return post({ type: "failed", err: result.err })
    }

    return post({ type: "success" })
}

interface Post<E, S> {
    (event: E): S
}
