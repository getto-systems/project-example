import { test, expect, describe } from "vitest"

import { observeAtom } from "../../../../../z_vendor/getto-atom/test_helper"
import { mockSingleBoardStore } from "../../../../../common/util/board/input/test_helper"

import { initAuthUserTextField } from "./action"

const fields = [{ name: "memo", maxLength: 255 }] as const

describe.each(fields)("$name", ({ name, maxLength }) => {
    test("validate; valid input", async () => {
        const { field, store } = standard()

        const result = observeAtom(field.validate)

        store.set("valid")

        expect(result()).toEqual([{ valid: true, value: "valid" }])
    })

    test("validate; invalid : too-long", async () => {
        const { field, store } = standard()

        const result = observeAtom(field.validate)

        store.set("a".repeat(maxLength + 1))

        expect(result()).toEqual([{ valid: false, err: [{ type: "too-long", maxLength }] }])
    })

    test("validate; valid : just max-length", async () => {
        const { field, store } = standard()

        const result = observeAtom(field.validate)

        store.set("a".repeat(maxLength))

        expect(result()).toEqual([{ valid: true, value: "a".repeat(maxLength) }])
    })

    test("observe; has changed", async () => {
        const { field, store } = standard()

        const result = observeAtom(field.observe)

        store.set("changed")

        expect(result()).toEqual([{ hasChanged: true }])
    })

    test("reset", () => {
        const { initializer, store } = standard()

        store.set("valid")
        initializer.reset()

        expect(store.get()).toEqual("")
    })

    function standard() {
        const [field, initializer] = initAuthUserTextField(name)
        const store = mockSingleBoardStore(field.input)

        return { field, initializer, store }
    }
})
