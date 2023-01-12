import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"

import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { TextFilter } from "../../search/kernel/data"

export interface TextFilterAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly observe: ObserveBoardFieldAction

    clear(): void
}

export function initTextFilterAction(initial: TextFilter): Readonly<{
    input: TextFilterAction
    pin: () => TextFilter
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({
            current: () => store.get(),
        }),
    })

    if (initial.filter) {
        store.set(initial.value)
    }

    subscriber.subscribe(() => {
        observe.check()
    })

    return {
        input: {
            input,
            observe,

            clear: () => {
                store.set("")
            },
        },
        pin: () => {
            observe.pin()

            const value = store.get()
            if (value === "") {
                return { filter: false }
            }
            return { filter: true, value }
        },
    }
}
