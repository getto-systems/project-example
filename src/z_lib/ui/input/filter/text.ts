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

export interface TextFilterAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly observe: ObserveBoardFieldAction

    clear(): void
}

export type TextFilterProps<T> = Readonly<{
    restore: (value: string) => T
}>
export function initTextFilterAction<T extends string>(
    props: TextFilterProps<T>,
): Readonly<{
    input: TextFilterAction
    pin: () => T
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

            clear: () => {
                store.set("")
            },
        },
        pin: () => {
            observe.pin()
            return props.restore(store.get())
        },
    }
}
