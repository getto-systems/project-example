import { ApplicationAction } from "../../../z_vendor/getto-application/action/action"

import {
    InputBoardAction,
    initInputBoardAction,
} from "../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../z_vendor/getto-application/board/observe_field/action"

import { seasonToString } from "../kernel/convert"

import { BoardValueStore } from "../../../z_vendor/getto-application/board/input/infra"

import { Season } from "../kernel/data"
import { initBoardFieldObserver } from "../../../z_vendor/getto-application/board/observe_field/init/observer"

export interface InputSeasonAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly observe: ObserveBoardFieldAction
}

export function initInputSeasonAction(): Readonly<{
    input: InputSeasonAction
    get: { (): string }
    set: { (season: Season): void }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    subscriber.subscribe(() => {
        observe.check()
    })

    return {
        input: {
            input,
            observe,
            terminate: () => {
                observe.terminate()
                subscriber.terminate()
            },
        },
        get: () => store.get(),
        set: (season) => store.set(seasonToString(season)),
    }
}
