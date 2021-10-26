import { initInputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/init"
import { initObserveBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_observe_field/init"
import { initBoardFieldObserver } from "../../../../../../ui/vendor/getto-application/board/observe_field/init/observer"

import { SearchLoginIDAction } from "./action"

import {
    BoardValue,
    emptyBoardValue,
} from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export function initSearchLoginIDAction(initial: BoardValue): Readonly<{
    input: SearchLoginIDAction
    pin: { (): BoardValue }
    peek: { (): BoardValue }
}> {
    const { input, store, subscriber } = initInputBoardAction()

    store.set(initial)

    const value = () => store.get()
    const observer = initBoardFieldObserver(value)
    const observe = initObserveBoardFieldAction({ observer })

    subscriber.subscribe(() => observe.check())

    return {
        input: {
            input,
            observe,
            clear: () => {
                store.set(emptyBoardValue)
                observe.check()
            },
            terminate: () => {
                subscriber.terminate()
            },
        },
        pin: () => {
            observer.pin()
            return value()
        },
        peek: () => {
            return observer.peek()
        },
    }
}
