import { test, expect } from "vitest"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initTextFilterAction } from "../../../../common/util/input/filter/text"

test("clear", () => {
    const { action, store } = standard()

    store.set("valid")
    action.clear()

    expect(store.get()).toEqual("")
})

function standard() {
    const { input: action } = initTextFilterAction({ filter: false })
    const store = mockBoardValueStore(action.input)

    return { action, store }
}
