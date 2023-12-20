import { Atom, initAtom } from "../../../z_vendor/getto-atom/atom"
import { LoadSeasonState } from "../load/action"
import { ValidateBoardState } from "../../../common/util/board/validate/action"
import { ObserveBoardState } from "../../../common/util/board/observe/action"
import {
    EditableBoardAction,
    initEditableBoardAction,
} from "../../../common/util/board/editable/action"
import {
    SelectFieldBoard,
    composeRegisterFieldBoard,
    initSelectFieldBoard,
} from "../../../common/util/board/field/action"

import { seasonToString } from "../kernel/convert"

import { SeasonRepository } from "../kernel/infra"
import { Clock } from "../../../common/util/clock/infra"
import { ExpireTime, WaitTime } from "../../../common/util/config/infra"

import { RepositoryError } from "../../../common/util/repository/data"
import { DetectedSeason, Season, defaultSeason, detectedSeason } from "../kernel/data"
import { ticker } from "../../../common/util/timer/helper"
import { ConvertBoardResult } from "../../../common/util/board/kernel/data"
import { loadState_loaded } from "../../../common/util/load/data"

export interface SetupSeasonAction {
    readonly state: Atom<SetupSeasonState>
    readonly season: SelectFieldBoard<DetectedSeason>
    readonly validate: Atom<ValidateBoardState>
    readonly observe: Atom<ObserveBoardState>
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
    const { state, post } = initAtom({ initialState })
    const editable = initEditableBoardAction()

    const seasonOptions = initAtom({
        initialState: loadState_loaded([
            defaultSeason(),
            ...material.infra.availableSeasons.map((season) => detectedSeason(season)),
        ]),
    })

    const season = initSelectFieldBoard(seasonOptions.state, {
        convert: (data) => seasonToString(data),
    })

    const convert = (): ConvertBoardResult<DetectedSeason> => {
        const result = {
            season: season[0].validate.currentState(),
        }
        if (!result.season.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: result.season.value,
        }
    }

    const { validate, observe, reset: _reset } = composeRegisterFieldBoard([season])

    load.state.ignitionState.then((state) => {
        switch (state.type) {
            case "success":
                season[1].init(state.season)
        }
    })

    onSuccess(() => {
        editable.close()
        load.load()
    })

    return {
        state,

        season: season[0],

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
