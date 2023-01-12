import { test, expect } from "vitest"

import { observeApplicationState } from "../../../../../z_vendor/getto-application/action/test_helper"
import { mockMultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initAuthPermissionGrantedFieldAction } from "./action"

import { ALL_AUTH_PERMISSIONS } from "../../../../../x_content/permission"

test("validate; valid input", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.validate.state, async () => {
            store.set(["auth-user"])
            return action.validate.state.currentState()
        }),
    ).toEqual([{ type: "validated", result: { valid: true, value: ["auth-user"] } }])
})

test("observe; has changed", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.observe.state, async () => {
            store.set(["auth-user"])
            return action.observe.state.currentState()
        }),
    ).toEqual([{ hasChanged: true }])
})

test("reset", () => {
    const { action, store } = standard()

    store.set(["auth-user"])
    action.reset([])

    expect(store.get()).toEqual([])
})

test("clear", () => {
    const { action, store } = standard()

    store.set(["auth-user"])
    action.clear()

    expect(store.get()).toEqual([])
})

function standard() {
    const { input, setOptions } = initAuthPermissionGrantedFieldAction()
    const store = mockMultipleBoardValueStore(input.input)

    setOptions(ALL_AUTH_PERMISSIONS)

    return { action: input, store }
}
