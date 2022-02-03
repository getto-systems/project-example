import { availableSeasons } from "../kernel/init/available_seasons"

import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../../ui/vendor/getto-application/action/action"
import { initInputSeasonAction, InputSeasonAction } from "../input/action"
import { LoadSeasonState } from "../load/action"

import { seasonBoardConverter, seasonToBoardValue } from "../kernel/convert"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime } from "../../../z_lib/ui/config/infra"

import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { RepositoryError } from "../../../z_lib/ui/repository/data"

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
    seasonRepository: SeasonRepository
    clock: Clock
}>
export type SetupSeasonConfig = Readonly<{
    manualSetupSeasonExpire: ExpireTime
}>

export type SetupSeasonState =
    | Readonly<{ type: "initial-setup" }>
    | Readonly<{ type: "edit-season" }>
    | SetupSeasonEvent

export const initialSetupSeasonState: SetupSeasonState = { type: "initial-setup" }

export function initSetupSeasonAction(
    material: SetupSeasonMaterial,
    loadState: Promise<LoadSeasonState>,
): SetupSeasonAction {
    return new Action(material, loadState)
}

class Action extends AbstractStatefulApplicationAction<SetupSeasonState> {
    readonly initialState = initialSetupSeasonState

    readonly season: InputSeasonAction

    material: SetupSeasonMaterial

    field: { (): BoardValue }

    constructor(material: SetupSeasonMaterial, loadState: Promise<LoadSeasonState>) {
        super()

        const season = initInputSeasonAction()

        this.season = season.input

        loadState.then((state) => {
            switch (state.type) {
                case "succeed-to-load":
                    if (!state.default) {
                        season.set(seasonToBoardValue(state.season))
                    }
                    return
            }
        })

        this.material = material
        this.field = () => season.get()
    }

    async setup(): Promise<SetupSeasonState> {
        return setupSeason(this.material, this.field(), this.post)
    }
    async open(): Promise<SetupSeasonState> {
        return this.post({ type: "edit-season" })
    }
}

type SetupSeasonEvent =
    | Readonly<{ type: "succeed-to-setup" }>
    | Readonly<{ type: "invalid-season" }>
    | Readonly<{ type: "failed-to-setup"; err: RepositoryError }>

async function setupSeason<S>(
    { infra, config }: SetupSeasonMaterial,
    value: BoardValue,
    post: Post<SetupSeasonEvent, S>,
): Promise<S> {
    const { clock, seasonRepository } = infra

    const convertResult = seasonBoardConverter(availableSeasons(clock), value)
    if (!convertResult.valid) {
        return post({ type: "invalid-season" })
    }

    if (convertResult.default) {
        const result = await seasonRepository.remove()
        if (!result.success) {
            return post({ type: "failed-to-setup", err: result.err })
        }
        return post({ type: "succeed-to-setup" })
    }

    const result = await seasonRepository.set({
        season: convertResult.season,
        expires: clock.now().getTime() + config.manualSetupSeasonExpire.expire_millisecond,
    })
    if (!result.success) {
        return post({ type: "failed-to-setup", err: result.err })
    }

    return post({ type: "succeed-to-setup" })
}

interface Post<E, S> {
    (event: E): S
}
