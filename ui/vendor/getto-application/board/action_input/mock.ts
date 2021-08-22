import { mockInputBoardValueAction } from "./core/mock"

import { InputBoardValueResource } from "./action"

import { BoardValueStore } from "../input/infra"

import { BoardValue, emptyBoardValue } from "../kernel/data"
import { InputBoardValueType } from "../input/data"

export function mockInputBoardValueResource(
    type: InputBoardValueType,
    value: BoardValue,
): InputBoardValueResource {
    return {
        type,
        input: mockInputBoardValueAction(value),
    }
}

export function mockBoardValueStore(): BoardValueStore {
    let store: BoardValue = emptyBoardValue
    return {
        get: () => store,
        set: (value) => {
            store = value
        },
    }
}
