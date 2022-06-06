import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initPasswordFieldAction } from "./action"

test("validate; valid input", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.set("valid")
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "validated", result: { valid: true, value: "valid" } }])
    })
})

test("validate; invalid : empty", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.set("")
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "validated", result: { valid: false, err: [{ type: "empty" }] } },
        ])
    })
})

test("validate; invalid : too-long", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.set("a".repeat(100 + 1))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: { valid: false, err: [{ type: "too-long", maxLength: 100 }] },
            },
        ])
    })
})

test("validate; valid : just max-length", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.set("a".repeat(100))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "validated", result: { valid: true, value: "a".repeat(100) } },
        ])
    })
})

test("validate; invalid : too-long : multi-byte", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.set("あ".repeat(100) + "a")
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: { valid: false, err: [{ type: "too-long", maxLength: 100 }] },
            },
        ])
    })
})

test("validate; valid : just max-length : multi-byte", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.set("あ".repeat(100))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "validated", result: { valid: true, value: "あ".repeat(100) } },
        ])
    })
})

test("password character state : single byte", () => {
    const { action, store } = standard()

    store.set("password")
    expect(action.checkCharacter()).toEqual({ multiByte: false })
})

test("password character state : multi byte", () => {
    const { action, store } = standard()

    store.set("パスワード")
    expect(action.checkCharacter()).toEqual({ multiByte: true })
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
