import { BoardValueStore, MultipleBoardValueStore } from "../infra"

import { BoardValue, emptyBoardValue } from "../../kernel/data"

export function mockBoardValueStore(): BoardValueStore {
    let store: BoardValue = emptyBoardValue
    return {
        get: () => store,
        set: (value) => {
            store = value
        },
    }
}

export function mockMultipleBoardValueStore(): MultipleBoardValueStore {
    let store: readonly BoardValue[] = []
    return {
        get: () => store,
        set: (value) => {
            store = value
        },
    }
}