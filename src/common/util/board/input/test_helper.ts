import { InputBoardAction } from "./action"

import { SingleBoardStore, MultipleBoardStore, FileBoardStore, SelectFileResult } from "./infra"

export function mockSingleBoardStore(input: InputBoardAction<SingleBoardStore>): SingleBoardStore {
    let storedValue = ""
    const store: SingleBoardStore = {
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
            input.onInput()
        },
    }
}

export function mockMultipleBoardStore(
    input: InputBoardAction<MultipleBoardStore>,
): MultipleBoardStore {
    let storedValue: readonly string[] = []
    const store: MultipleBoardStore = {
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
            input.onInput()
        },
    }
}

export function mockFileBoardStore(
    input: InputBoardAction<FileBoardStore>,
    result: SelectFileResult,
): FileBoardStore {
    const store: FileBoardStore = {
        get: () => result,
    }

    input.connector.connect(store)

    return store
}
