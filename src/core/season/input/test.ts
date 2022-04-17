import { markBoardValue } from "../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../z_vendor/getto-application/board/input/test_helper"

import { initInputSeasonAction } from "./action"

import { Season } from "../kernel/data"

test("get value", async () => {
    const { store, get } = standard()

    store.set(markBoardValue("2021.summer"))
    expect(get()).toEqual("2021.summer")
})

test("set value", async () => {
    const { store, set } = standard()

    set({ year: 2021, period: "summer" } as Season)
    expect(store.get()).toEqual("2021.summer")
})

test("terminate", async () => {
    const { action } = standard()

    action.terminate()

    // input action では特に subscribe しないので確認することがない
    expect(true).toBe(true)
})

function standard() {
    const { input: action, get, set } = initInputSeasonAction()
    const store = mockBoardValueStore(action.input)

    return { action, store, get, set }
}
