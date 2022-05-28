import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../../../z_vendor/getto-application/action/test_helper"

import { mockBoardValueStore } from "../../../../../../z_vendor/getto-application/board/input/test_helper"

import { initInputResetTokenDestinationAction } from "./action"

import { restoreResetTokenDestination } from "../kernel/convert"

import { ResetTokenDestination } from "../kernel/data"

test("validate; valid input", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set("email")
        store.email.set("user@example.com")
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: { valid: true, value: { type: "email", email: "user@example.com" } },
            },
        ])
    })
})

test("validate; invalid : empty", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set("email")
        store.email.set("")
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
            },
        ])
    })
})

test("validate; invalid : invalid", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set("email")
        store.email.set("invalid-email; not includes at-mark")
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: { valid: false, err: { type: "email", err: [{ type: "invalid-email" }] } },
            },
        ])
    })
})

test("validate; invalid : too-long", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set("email")
        store.email.set("@".repeat(255 + 1))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: {
                    valid: false,
                    err: { type: "email", err: [{ type: "too-long", maxLength: 255 }] },
                },
            },
        ])
    })
})

test("validate; valid : just max-length", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.destinationType.set("email")
        store.email.set("@".repeat(255))
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            {
                type: "validated",
                result: {
                    valid: true,
                    value: { type: "email", email: "@".repeat(255) },
                },
            },
        ])
    })
})

test("reset; has email", () => {
    const { action, store } = standard()
    const destination = standard_destination()

    store.email.set("some-user@example.com")
    action.reset(destination)

    expect(store.destinationType.get()).toEqual("email")
    expect(store.email.get()).toEqual("user@example.com")
})
test("reset; none", () => {
    const { action, store } = standard()
    const destination = no_destination()

    store.email.set("some-user@example.com")
    action.reset(destination)

    expect(store.destinationType.get()).toEqual("none")
    expect(store.email.get()).toEqual("")
})

function standard() {
    const action = initInputResetTokenDestinationAction()
    const store = {
        destinationType: mockBoardValueStore(action.destinationType),
        email: mockBoardValueStore(action.email),
    }

    return { action, store }
}

function standard_destination(): ResetTokenDestination {
    return restoreResetTokenDestination({ type: "email", email: "user@example.com" })
}
function no_destination(): ResetTokenDestination {
    return restoreResetTokenDestination({ type: "none", email: "" })
}
