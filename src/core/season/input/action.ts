import { ApplicationAction } from "../../../z_vendor/getto-application/action/action"

import {
    InputBoardAction,
    initInputBoardAction,
} from "../../../z_vendor/getto-application/board/input/action"

import { seasonToBoardValue } from "../kernel/convert"

import { BoardValue } from "../../../z_vendor/getto-application/board/kernel/data"
import { Season } from "../kernel/data"

export interface InputSeasonAction extends ApplicationAction {
    readonly input: InputBoardAction
}

export function initInputSeasonAction(): Readonly<{
    input: InputSeasonAction
    get: { (): BoardValue }
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
        set: (season) => store.set(seasonToBoardValue(season)),
    }
}
