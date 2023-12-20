import { test, expect } from "vitest"
import { observeAtom } from "../../../../../z_vendor/getto-atom/test_helper"
import { mockSingleBoardStore } from "../../../../../common/util/board/input/test_helper"

import { initPasswordField } from "./action"

test("validate; valid input", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.set("valid")

    expect(result()).toEqual([{ valid: true, value: "valid" }])
})

test("validate; invalid : empty", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.set("")

    expect(result()).toEqual([{ valid: false, err: [{ type: "empty" }] }])
})

test("validate; invalid : too-long", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.set("a".repeat(100 + 1))

    expect(result()).toEqual([{ valid: false, err: [{ type: "too-long", maxLength: 100 }] }])
})

test("validate; valid : just max-length", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.set("a".repeat(100))

    expect(result()).toEqual([{ valid: true, value: "a".repeat(100) }])
})

test("validate; invalid : too-long : multi-byte", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.set("あ".repeat(100) + "a")

    expect(result()).toEqual([{ valid: false, err: [{ type: "too-long", maxLength: 100 }] }])
})

test("validate; valid : just max-length : multi-byte", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.validate)

    store.set("あ".repeat(100))

    expect(result()).toEqual([{ valid: true, value: "あ".repeat(100) }])
})

test("password character state : single byte", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.character)

    store.set("password")

    expect(result()).toEqual([{ multiByte: false }])
})

test("password character state : multi byte", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.character)

    store.set("パスワード")

    expect(result()).toEqual([{ multiByte: true }])
})

test("reset", () => {
    const { initializer, store } = standard()

    store.set("valid")
    initializer.reset()

    expect(store.get()).toEqual("")
})

function standard() {
    const [field, initializer] = initPasswordField()
    const store = mockSingleBoardStore(field.input)

    return { field, initializer, store }
}
