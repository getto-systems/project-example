import { initInputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/init"
import { initObserveBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_observe_field/init"
import { initBoardFieldObserver } from "../../../../../../ui/vendor/getto-application/board/observe_field/init/observer"

import { SearchOffsetAction } from "./action"

import {
    BoardValue,
    zeroBoardValue,
} from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export function initSearchOffsetAction(initial: BoardValue): Readonly<{
    input: SearchOffsetAction
    pin: { (): BoardValue }
    reset: { (): BoardValue }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    store.set(initial)

    const storeValue = () => store.get()
    const observer = initBoardFieldObserver(storeValue)
    const observe = initObserveBoardFieldAction({ observer })

    subscriber.subscribe(() => observe.check())

    return {
        input: {
            input,
            observe,
            terminate: () => {
                subscriber.terminate()
            },
        },
        pin: () => {
            observer.pin()
            return storeValue()
        },
        reset: () => {
            store.set(zeroBoardValue)
            observer.pin()
            return storeValue()
        },
    }
}
