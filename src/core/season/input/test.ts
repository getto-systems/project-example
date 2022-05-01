import { test, expect } from "vitest"
import { mockBoardValueStore } from "../../../z_vendor/getto-application/board/input/test_helper"

import { initInputSeasonAction } from "./action"

import { Season } from "../kernel/data"
import { markSeason } from "../kernel/test_helper"

test("get value", async () => {
    const { action, store } = standard()

    store.set("2021.summer")
    expect(action.validate.check()).toEqual({
        valid: true,
        value: { default: false, season: { year: 2021, period: "summer" } },
    })
})

test("get default", async () => {
    const { action, store } = standard()

    store.set("")
    expect(action.validate.check()).toEqual({
        valid: true,
        value: { default: true },
    })
})

test("set value", async () => {
    const { action, store } = standard()

    action.reset(markSeason({ year: 2021, period: "summer" }))
    expect(store.get()).toEqual("2021.summer")
})

test("terminate", async () => {
    const { action } = standard()

    action.terminate()

    // input action では特に subscribe しないので確認することがない
    expect(true).toBe(true)
})

function standard() {
    const action = initInputSeasonAction(standard_availableSeasons())
    const store = mockBoardValueStore(action.input)

    return { action, store }
}

function standard_availableSeasons(): readonly Season[] {
    return [markSeason({ year: 2021, period: "summer" })]
}
