import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initLoginIdFieldAction } from "./action"

test("validate; valid input", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.validate.state, async () => {
            store.set("valid")
            return action.validate.state.currentState()
        }),
    ).toEqual([{ type: "validated", result: { valid: true, value: "valid" } }])
})

test("validate; invalid : empty", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.validate.state, async () => {
            store.set("")
            return action.validate.state.currentState()
        }),
    ).toEqual([{ type: "validated", result: { valid: false, err: [{ type: "empty" }] } }])
})

test("validate; invalid : too-long", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.validate.state, async () => {
            store.set("a".repeat(100 + 1))
            return action.validate.state.currentState()
        }),
    ).toEqual([
        {
            type: "validated",
            result: { valid: false, err: [{ type: "too-long", maxLength: 100 }] },
        },
    ])
})

test("validate; valid : just max-length", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.validate.state, async () => {
            store.set("a".repeat(100))
            return action.validate.state.currentState()
        }),
    ).toEqual([{ type: "validated", result: { valid: true, value: "a".repeat(100) } }])
})

test("clear", () => {
    const { action, store } = standard()

    store.set("valid")
    action.clear()

    expect(store.get()).toEqual("")
})

function standard() {
    const action = initLoginIdFieldAction()
    const store = mockBoardValueStore(action.input)

    return { action, store }
}
