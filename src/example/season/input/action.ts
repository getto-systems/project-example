import { ApplicationAction } from "../../../../ui/vendor/getto-application/action/action"

import {
    InputBoardAction,
    initInputBoardAction,
} from "../../../../ui/vendor/getto-application/board/input/action"

import {
    BoardValue,
    emptyBoardValue,
} from "../../../../ui/vendor/getto-application/board/kernel/data"

export interface InputSeasonAction extends ApplicationAction {
    readonly input: InputBoardAction
}

export function initInputSeasonAction(): Readonly<{
    input: InputSeasonAction
    get: { (): BoardValue }
    set: { (value: BoardValue): void }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    store.set(emptyBoardValue)

    return {
        input: {
            input,
            terminate: () => {
                subscriber.terminate()
            },
        },
        get: () => store.get(),
        set: (value) => store.set(value),
    }
}
