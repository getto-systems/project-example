import { initInputBoardValueAction } from "./core/impl"

import { InputBoardValueResource } from "./action"

import { InputBoardValueType } from "../input/data"

export function initInputBoardValueResource(type: InputBoardValueType): InputBoardValueResource {
    return {
        type,
        input: initInputBoardValueAction(),
    }
}
