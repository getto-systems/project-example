import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"

import { mockBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initInputAuthUserMemoAction } from "./action"

import { restoreAuthUserMemo } from "./convert"

test("validate; valid input", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.memo.set("valid")
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "validated", result: { valid: true, value: "valid" } }])
    })
})

test("validate; invalid : too-long", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.memo.set("a".repeat(255 + 1))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: { valid: false, err: [{ type: "too-long", maxLength: 255 }] },
            },
        ])
    })
})

test("validate; valid : just max-length", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.memo.set("a".repeat(255))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "validated", result: { valid: true, value: "a".repeat(255) } },
        ])
    })
})

test("observe; has changed", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.observe.subscriber)

    await runner(async () => {
        store.memo.set("changed")
        return action.observe.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: true }])
    })
})

test("reset", () => {
    const { action, store } = standard()

    store.memo.set("valid")
    action.reset(restoreAuthUserMemo(""))

    expect(store.memo.get()).toEqual("")
})

test("terminate", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            action.observe.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        action.terminate()
        return action.observe.currentState()
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    const action = initInputAuthUserMemoAction()
    const store = {
        memo: mockBoardValueStore(action.input),
    }

    return { action, store }
}
