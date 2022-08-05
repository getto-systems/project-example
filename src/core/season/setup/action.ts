import {
    ApplicationState,
    initApplicationState,
    StatefulApplicationAction,
} from "../../../z_vendor/getto-application/action/action"
import { initSeasonFieldAction, SeasonFieldAction } from "../input/action"
import { LoadSeasonState } from "../load/action"
import { ObserveBoardAction } from "../../../z_vendor/getto-application/board/observe_board/action"
import { ValidateBoardAction } from "../../../z_vendor/getto-application/board/validate_board/action"
import { initRegisterField } from "../../../z_lib/ui/register/action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime, WaitTime } from "../../../z_lib/ui/config/infra"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { DetectedSeason, Season } from "../kernel/data"
import { ConvertBoardResult } from "../../../z_vendor/getto-application/board/kernel/data"
import { ticker } from "../../../z_lib/ui/timer/helper"

export interface SetupSeasonAction extends StatefulApplicationAction<SetupSeasonState> {
    readonly season: SeasonFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    setup(onSuccess: { (): void }): Promise<SetupSeasonState>
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
    resetToInitialTimeout: WaitTime
}>

export type SetupSeasonState = SetupSeasonEvent

const initialState: SetupSeasonState = { type: "initial" }

export function initSetupSeasonAction(
    material: SetupSeasonMaterial,
    load: LoadAction,
): SetupSeasonAction {
    return new Action(material, load)
}

interface LoadAction {
    readonly state: { ignitionState: Promise<LoadSeasonState> }
    load(): Promise<LoadSeasonState>
}

class Action implements SetupSeasonAction {
    readonly material: SetupSeasonMaterial
    readonly state: ApplicationState<SetupSeasonState>
    readonly post: (state: SetupSeasonState) => SetupSeasonState

    readonly season: SeasonFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    load: LoadAction

    convert: { (): ConvertBoardResult<DetectedSeason> }

    constructor(material: SetupSeasonMaterial, load: LoadAction) {
        const { state, post } = initApplicationState({ initialState })
        this.state = state
        this.post = post

        const season = initSeasonFieldAction(material.infra.availableSeasons)

        const convert = (): ConvertBoardResult<DetectedSeason> => {
            const result = {
                season: season.validate.check(),
            }
            if (!result.season.valid) {
                return { valid: false }
            }
            return {
                valid: true,
                value: result.season.value,
            }
        }

        const { validate, observe } = initRegisterField([["season", season]], convert)

        load.state.ignitionState.then((state) => {
            switch (state.type) {
                case "success":
                    if (state.default) {
                        season.reset({ default: true })
                    } else {
                        season.reset({ default: false, season: state.season })
                    }
            }
        })

        this.season = season
        this.validate = validate
        this.observe = observe

        this.material = material
        this.load = load
        this.convert = convert
    }

    async setup(onSuccess: { (): void }): Promise<SetupSeasonState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.state.currentState()
        }
        return setupSeason(
            this.material,
            fields.value,
            () => {
                onSuccess()
                this.load.load()
            },
            this.post,
        )
    }
}

type SetupSeasonEvent =
    | Readonly<{ type: "failed"; err: RepositoryError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function setupSeason<S>(
    { infra, config }: SetupSeasonMaterial,
    season: DetectedSeason,
    onSuccess: { (): void },
    post: Post<SetupSeasonEvent, S>,
): Promise<S> {
    const { clock, seasonRepository } = infra

    if (season.default) {
        const result = await seasonRepository.remove()
        if (!result.success) {
            return post({ type: "failed", err: result.err })
        }

        onSuccess()
        post({ type: "success" })
        return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
    }

    const result = await seasonRepository.set({
        season: season.season,
        expires: clock.now().getTime() + config.manualSetupSeasonExpire.expire_millisecond,
    })
    if (!result.success) {
        return post({ type: "failed", err: result.err })
    }

    onSuccess()
    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
