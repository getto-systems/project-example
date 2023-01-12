import {
    ApplicationState,
    initApplicationState,
} from "../../../z_vendor/getto-application/action/action"
import { initSeasonFieldAction, SeasonFieldAction } from "../input/action"
import { LoadSeasonState } from "../load/action"
import { ObserveBoardAction } from "../../../z_vendor/getto-application/board/observe_board/action"
import { ValidateBoardAction } from "../../../z_vendor/getto-application/board/validate_board/action"
import {
    EditableBoardAction,
    initEditableBoardAction,
} from "../../../z_vendor/getto-application/board/editable/action"
import { initRegisterField } from "../../../common/util/register/action"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../common/util/clock/infra"
import { ExpireTime, WaitTime } from "../../../common/util/config/infra"

import { RepositoryError } from "../../../common/util/repository/data"
import { DetectedSeason, Season } from "../kernel/data"
import { ConvertBoardResult } from "../../../z_vendor/getto-application/board/kernel/data"
import { ticker } from "../../../common/util/timer/helper"

export interface SetupSeasonAction {
    readonly state: ApplicationState<SetupSeasonState>
    readonly season: SeasonFieldAction
    readonly validate: ValidateBoardAction
    readonly observe: ObserveBoardAction
    readonly editable: EditableBoardAction

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
    resetToInitialTimeout: WaitTime
}>

export type SetupSeasonState = SetupSeasonEvent

const initialState: SetupSeasonState = { type: "initial" }

interface LoadAction {
    readonly state: { ignitionState: Promise<LoadSeasonState> }
    load(): Promise<LoadSeasonState>
}

export function initSetupSeasonAction(
    material: SetupSeasonMaterial,
    load: LoadAction,
): SetupSeasonAction {
    const { state, post } = initApplicationState({ initialState })
    const editable = initEditableBoardAction()

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

    onSuccess(() => {
        editable.close()
        load.load()
    })

    return {
        state,

        season,

        validate,
        observe,
        editable,

        async setup(): Promise<SetupSeasonState> {
            const fields = convert()
            if (!fields.valid) {
                return state.currentState()
            }
            return setupSeason(material, fields.value, post)
        },
    }

    function onSuccess(handler: () => void): void {
        state.subscribe((state) => {
            switch (state.type) {
                case "success":
                    handler()
                    break
            }
        })
    }
}

type SetupSeasonEvent =
    | Readonly<{ type: "failed"; err: RepositoryError }>
    | Readonly<{ type: "success" }>
    | Readonly<{ type: "initial" }>

async function setupSeason<S>(
    { infra, config }: SetupSeasonMaterial,
    season: DetectedSeason,
    post: Post<SetupSeasonEvent, S>,
): Promise<S> {
    const { clock, seasonRepository } = infra

    if (season.default) {
        const result = await seasonRepository.remove()
        if (!result.success) {
            return post({ type: "failed", err: result.err })
        }
    } else {
        const result = await seasonRepository.set({
            season: season.season,
            expires: clock.now().getTime() + config.manualSetupSeasonExpire.expire_millisecond,
        })
        if (!result.success) {
            return post({ type: "failed", err: result.err })
        }
    }

    post({ type: "success" })
    return ticker(config.resetToInitialTimeout, () => post({ type: "initial" }))
}

interface Post<E, S> {
    (event: E): S
}
