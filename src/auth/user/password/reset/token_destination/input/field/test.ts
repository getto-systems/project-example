import { test, expect } from "vitest"
import { observeAtom } from "../../../../../../../z_vendor/getto-atom/test_helper"

import { initAtom } from "../../../../../../../z_vendor/getto-atom/atom"
import { LoadState, loadState_loaded } from "../../../../../../../common/util/load/data"
import { mockSingleBoardStore } from "../../../../../../../common/util/board/input/test_helper"
import { initResetTokenDestinationField } from "./action"

import { ResetTokenDestination, resetTokenDestinationTypeVariants } from "../../kernel/data"

const maxLength = 255

test("validate; valid input", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.type.set("email")
    store.email.set("valid@example.com")

    expect(result()).toEqual([
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: true, value: { type: "email", email: "valid@example.com" } },
    ])
})

test("validate; invalid : email", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.type.set("email")
    store.email.set("invalid-email")

    expect(result()).toEqual([
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: false, err: { type: "email", err: [{ type: "invalid-email" }] } },
    ])
})

test("validate; invalid : too-long", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.type.set("email")
    store.email.set("@".repeat(maxLength + 1))

    expect(result()).toEqual([
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: false, err: { type: "email", err: [{ type: "too-long", maxLength }] } },
    ])
})

test("validate; valid : just max-length", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.type.set("email")
    store.email.set("@".repeat(maxLength))

    expect(result()).toEqual([
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: false, err: { type: "email", err: [{ type: "empty" }] } },
        { valid: true, value: { type: "email", email: "@".repeat(maxLength) } },
    ])
})

test("validate; valid : type-none", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.type.set("none")
    store.email.set("invalid-email")

    expect(result()).toEqual([
        { valid: true, value: { type: "none" } },
        { valid: true, value: { type: "none" } },
        { valid: true, value: { type: "none" } },
    ])
})

test("observe; has changed", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.observe)

    store.type.set("email")
    store.email.set("changed@example.com")

    expect(result()).toEqual([{ hasChanged: true }, { hasChanged: true }, { hasChanged: true }])
})

test("reset", () => {
    const { initializer, store } = standard()

    store.type.set("email")
    store.email.set("valid@example.com")
    initializer.reset()

    expect(store.type.get()).toEqual("")
    expect(store.email.get()).toEqual("")

    store.type.set("email")
    store.email.set("valid@example.com")
    initializer.pin()
    initializer.reset()

    expect(store.type.get()).toEqual("email")
    expect(store.email.get()).toEqual("valid@example.com")
})

function standard() {
    const options = initAtom<LoadState<readonly ResetTokenDestination["type"][]>>({
        initialState: loadState_loaded(resetTokenDestinationTypeVariants),
    })
    const [field, initializer] = initResetTokenDestinationField(options.state)
    const store = {
        type: mockSingleBoardStore(field.type.input),
        email: mockSingleBoardStore(field.email.input),
    }

    return { field, initializer, store }
}
