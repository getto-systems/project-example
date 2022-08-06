import { test, expect, describe } from "vitest"

import { observeApplicationState } from "../../../../../z_vendor/getto-application/action/test_helper"
import { mockBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"
import { restoreAuthUserField } from "../../kernel/convert"

import { initAuthUserTextFieldAction } from "./action"

const fields = [{ name: "memo", maxLength: 255 }] as const

describe.each(fields)("$name", ({ name, maxLength }) => {
    test("validate; valid input", async () => {
        const { action, store } = standard()

        expect(
            await observeApplicationState(action.validate.state, async () => {
                store.set("valid")
                return action.validate.state.currentState()
            }),
        ).toEqual([{ type: "validated", result: { valid: true, value: "valid" } }])
    })

    test("validate; invalid : too-long", async () => {
        const { action, store } = standard()

        expect(
            await observeApplicationState(action.validate.state, async () => {
                store.set("a".repeat(maxLength + 1))
                return action.validate.state.currentState()
            }),
        ).toEqual([
            {
                type: "validated",
                result: { valid: false, err: [{ type: "too-long", maxLength }] },
            },
        ])
    })

    test("validate; valid : just max-length", async () => {
        const { action, store } = standard()

        expect(
            await observeApplicationState(action.validate.state, async () => {
                store.set("a".repeat(maxLength))
                return action.validate.state.currentState()
            }),
        ).toEqual([{ type: "validated", result: { valid: true, value: "a".repeat(maxLength) } }])
    })

    test("observe; has changed", async () => {
        const { action, store } = standard()

        expect(
            await observeApplicationState(action.observe.state, async () => {
                store.set("changed")
                return action.observe.state.currentState()
            }),
        ).toEqual([{ hasChanged: true }])
    })

    test("reset", () => {
        const { action, store } = standard()

        store.set("valid")
        action.reset(restoreAuthUserField(""))

        expect(store.get()).toEqual("")
    })

    test("clear", () => {
        const { action, store } = standard()

        store.set("valid")
        action.clear()

        expect(store.get()).toEqual("")
    })

    function standard() {
        const action = initAuthUserTextFieldAction(name)
        const store = mockBoardValueStore(action.input)

        return { action, store }
    }
})
