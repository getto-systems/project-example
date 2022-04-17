import { InputBoardAction } from "./action"

import { BoardValueStore, MultipleBoardValueStore, FileStore, SelectFileResult } from "./infra"

import { BoardValue, emptyBoardValue } from "../kernel/data"

export function mockBoardValueStore(input: InputBoardAction<BoardValueStore>): BoardValueStore {
    let storedValue: BoardValue = emptyBoardValue
    const store: BoardValueStore = {
        get: () => storedValue,
        set: (value) => {
            storedValue = value
        },
    }

    input.connector.connect(store)

    return {
        get: () => store.get(),
        set: (value) => {
            store.set(value)
            input.publisher.post()
        },
    }
}

export function mockMultipleBoardValueStore(
    input: InputBoardAction<MultipleBoardValueStore>,
): MultipleBoardValueStore {
    let storedValue: readonly BoardValue[] = []
    const store: MultipleBoardValueStore = {
        get: () => storedValue,
        set: (value) => {
            storedValue = value
        },
    }

    input.connector.connect(store)

    return {
        get: () => store.get(),
        set: (value) => {
            store.set(value)
            input.publisher.post()
        },
    }
}

export function mockFileStore(
    input: InputBoardAction<FileStore>,
    result: SelectFileResult,
): FileStore {
    const store: FileStore = {
        get: () => result,
    }

    input.connector.connect(store)

    return store
}
