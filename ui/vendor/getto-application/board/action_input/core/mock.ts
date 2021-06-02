import { InputBoardValueAction } from "./action"

import { BoardValue } from "../../kernel/data"

export function mockInputBoardValueAction(value: BoardValue): InputBoardValueAction {
    return {
        storeLinker: {
            link: () => null,
            unlink: () => null,
        },

        subscribeInputEvent: () => null,
        triggerInputEvent: () => null,

        get: () => value,
        set: () => null,
        clear: () => null,

        terminate: () => null,
    }
}
