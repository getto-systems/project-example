import { test, expect } from "vitest"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { initSearchOffsetAction, SearchOffsetAction } from "./action"

test("get; reset", async () => {
    const { store, get, reset } = standard()

    store.offset.set("1")

    expect(get()).toEqual("1")
    expect(reset()).toEqual("0")
    expect(get()).toEqual("0")
})

function standard() {
    return initResource()
}

function initResource(): Readonly<{
    resource: Readonly<{ field: SearchOffsetAction }>
    get: { (): string }
    reset: { (): string }
    store: Readonly<{
        offset: BoardValueStore
    }>
}> {
    const { input, get, reset } = initSearchOffsetAction("0")
    const resource = {
        field: input,
    }

    const store = {
        offset: mockBoardValueStore(resource.field.input),
    }

    return { resource, store, get, reset }
}
