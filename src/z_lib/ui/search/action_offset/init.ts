import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import { SearchOffsetAction } from "./action"

import {
    BoardValue,
    zeroBoardValue,
} from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function initSearchOffsetAction(initial: BoardValue): Readonly<{
    input: SearchOffsetAction
    get: { (): BoardValue }
    reset: { (): BoardValue }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    store.set(initial)

    const storeValue = () => store.get()

    return {
        input: {
            input,
            terminate: () => {
                subscriber.terminate()
            },
        },
        get: () => {
            return storeValue()
        },
        reset: () => {
            store.set(zeroBoardValue)
            return storeValue()
        },
    }
}
