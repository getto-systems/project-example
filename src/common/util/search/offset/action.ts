import {
    InputBoardAction,
    initInputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

export interface SearchOffsetAction {
    readonly input: InputBoardAction<BoardValueStore>
}

export function initSearchOffsetAction(initial: string): Readonly<{
    input: SearchOffsetAction
    get: { (): string }
    reset: { (): string }
}> {
    const { input, store } = initInputBoardAction()

    store.set(initial)

    const storeValue = () => store.get()

    return {
        input: {
            input,
        },
        get: () => {
            return storeValue()
        },
        reset: () => {
            store.set("0")
            return storeValue()
        },
    }
}
