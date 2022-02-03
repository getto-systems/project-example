import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/input/action"
import { InputSeasonAction } from "./action"

import {
    BoardValue,
    emptyBoardValue,
} from "../../../../../ui/vendor/getto-application/board/kernel/data"

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
