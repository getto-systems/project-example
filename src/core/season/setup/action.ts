import {
    AbstractStatefulApplicationAction,
    StatefulApplicationAction,
} from "../../../z_vendor/getto-application/action/action"
import { initInputSeasonAction, InputSeasonAction } from "../input/action"
import { LoadSeasonState } from "../load/action"
import {
    initObserveBoardAction,
    ObserveBoardAction,
} from "../../../z_vendor/getto-application/board/observe_board/action"
import {
    initValidateBoardAction,
    ValidateBoardAction,
} from "../../../z_vendor/getto-application/board/validate_board/action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { ExpireTime, WaitTime } from "../../../z_lib/ui/config/infra"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { DetectedSeason, Season } from "../kernel/data"
import { ConvertBoardResult } from "../../../z_vendor/getto-application/board/kernel/data"
import { ticker } from "../../../z_lib/ui/timer/helper"

export interface SetupSeasonAction extends StatefulApplicationAction<SetupSeasonState> {
    readonly season: InputSeasonAction
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
    ignitionState: Promise<LoadSeasonState>
    load(): Promise<LoadSeasonState>
}

class Action extends AbstractStatefulApplicationAction<SetupSeasonState> {
    readonly initialState = initialState

    readonly season: InputSeasonAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction

    material: SetupSeasonMaterial
    load: LoadAction

    convert: { (): ConvertBoardResult<DetectedSeason> }

    constructor(material: SetupSeasonMaterial, load: LoadAction) {
        super()

        const season = initInputSeasonAction(material.infra.availableSeasons)

        const fields = ["season"] as const
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

        const { validate, validateChecker } = initValidateBoardAction({ fields }, { convert })
        const { observe, observeChecker } = initObserveBoardAction({ fields })

        load.ignitionState.then((state) => {
            switch (state.type) {
                case "success":
                    season.reset(state.season)
            }
        })

        this.season = season
        this.validate = validate
        this.observe = observe

        this.material = material
        this.load = load
        this.convert = convert

        fields.forEach((field) => {
            this[field].validate.subscriber.subscribe((state) => {
                validateChecker.update(field, state)
            })
            this[field].observe.subscriber.subscribe((result) => {
                observeChecker.update(field, result.hasChanged)
            })
        })
    }

    async setup(onSuccess: { (): void }): Promise<SetupSeasonState> {
        const fields = this.convert()
        if (!fields.valid) {
            return this.currentState()
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
