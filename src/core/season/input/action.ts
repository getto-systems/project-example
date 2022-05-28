import {
    InputBoardAction,
    initInputBoardAction,
} from "../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../z_vendor/getto-application/board/observe_field/action"

import { seasonConverter, seasonToString } from "../kernel/convert"

import { BoardValueStore } from "../../../z_vendor/getto-application/board/input/infra"

import { DetectedSeason, Season, ValidateSeasonError } from "../kernel/data"
import { initBoardFieldObserver } from "../../../z_vendor/getto-application/board/observe_field/init/observer"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
} from "../../../z_vendor/getto-application/board/validate_field/action"

export interface InputSeasonAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<DetectedSeason, ValidateSeasonError>
    readonly observe: ObserveBoardFieldAction

    reset(season: Season): void
}

export function initInputSeasonAction(availableSeasons: readonly Season[]): InputSeasonAction {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        convert: () => seasonConverter(availableSeasons, store.get()),
    })
    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    subscriber.subscribe(() => {
        validate.check()
        observe.check()
    })

    return {
        input,
        validate,
        observe,
        reset: (season) => {
            store.set(seasonToString(season))
            observe.pin()
        },
    }
}
