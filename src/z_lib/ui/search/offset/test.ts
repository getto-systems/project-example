import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { initSearchOffsetAction, SearchOffsetAction } from "./action"
import {
    BoardValue,
    zeroBoardValue,
} from "../../../../z_vendor/getto-application/board/kernel/data"

test("get; reset", async () => {
    const { store, get, reset } = standard()

    store.offset.set(markBoardValue("1"))

    expect(get()).toEqual("1")
    expect(reset()).toEqual("0")
    expect(get()).toEqual("0")
})

test("terminate", async () => {
    const { resource } = standard()

    resource.field.terminate()

    // offset action では subscribe していないのでテストする内容がない
    expect(true).toBe(true)
})

function standard() {
    return initResource()
}

function initResource(): Readonly<{
    resource: Readonly<{ field: SearchOffsetAction }>
    get: { (): BoardValue }
    reset: { (): BoardValue }
    store: Readonly<{
        offset: BoardValueStore
    }>
}> {
    const { input, get, reset } = initSearchOffsetAction(zeroBoardValue)
    const resource = {
        field: input,
    }

    const store = {
        offset: mockBoardValueStore(resource.field.input),
    }

    return { resource, store, get, reset }
}
