import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initPasswordFieldAction } from "./action"
import { Password } from "./data"

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

test("validate; invalid : too-long : multi-byte", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.validate.state, async () => {
            store.set("あ".repeat(100) + "a")
            return action.validate.state.currentState()
        }),
    ).toEqual([
        {
            type: "validated",
            result: { valid: false, err: [{ type: "too-long", maxLength: 100 }] },
        },
    ])
})

test("validate; valid : just max-length : multi-byte", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.validate.state, async () => {
            store.set("あ".repeat(100))
            return action.validate.state.currentState()
        }),
    ).toEqual([{ type: "validated", result: { valid: true, value: "あ".repeat(100) } }])
})

test("password character state : single byte", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.character.state, async () => {
            store.set("password")
            return action.character.state.currentState()
        }),
    ).toEqual([{ multiByte: false }])
})

test("password character state : multi byte", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.character.state, async () => {
            store.set("パスワード")
            return action.character.state.currentState()
        }),
    ).toEqual([{ multiByte: true }])
})

test("reset", () => {
    const { action, store } = standard()

    store.set("valid")
    action.reset("password" as Password)

    expect(store.get()).toEqual("password")
})

test("clear", () => {
    const { action, store } = standard()

    store.set("valid")
    action.clear()

    expect(store.get()).toEqual("")
})

function standard() {
    const action = initPasswordFieldAction()
    const store = mockBoardValueStore(action.input)

    return { action, store }
}
