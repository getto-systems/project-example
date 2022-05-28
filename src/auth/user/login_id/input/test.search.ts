import { test, expect } from "vitest"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initFilterLoginIdAction } from "./action"

test("clear", () => {
    const { action, store } = standard()

    store.set("valid")
    action.clear()

    expect(store.get()).toEqual("")
})

function standard() {
    const { input: action } = initFilterLoginIdAction({ filter: false })
    const store = mockBoardValueStore(action.input)

    return { action, store }
}
