import { BoardValueStore } from "../infra"

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
