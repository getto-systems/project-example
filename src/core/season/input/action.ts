import { ApplicationAction } from "../../../z_vendor/getto-application/action/action"

import {
    InputBoardAction,
    initInputBoardAction,
} from "../../../z_vendor/getto-application/board/input/action"

import { seasonToString } from "../kernel/convert"

import { BoardValueStore } from "../../../z_vendor/getto-application/board/input/infra"

import { Season } from "../kernel/data"

export interface InputSeasonAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
}

export function initInputSeasonAction(): Readonly<{
    input: InputSeasonAction
    get: { (): string }
    set: { (season: Season): void }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    return {
        input: {
            input,
            terminate: () => {
                subscriber.terminate()
            },
        },
        get: () => store.get(),
        set: (season) => store.set(seasonToString(season)),
    }
}
