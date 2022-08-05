import { test, expect } from "vitest"

import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"
import { mockMultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initAuthUserGrantedRolesFieldAction } from "./action"

import { ALL_AUTH_ROLES } from "../../../../../x_content/role"

test("validate; valid input", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.state)

    await runner(async () => {
        store.set(["auth-user"])
        return action.validate.state.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "validated", result: { valid: true, value: ["auth-user"] } },
        ])
    })
})

test("observe; has changed", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.observe.state)

    await runner(async () => {
        store.set(["auth-user"])
        return action.observe.state.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: true }])
    })
})

test("options", () => {
    const { action } = standard()

    expect(action.options()).toEqual({ isLoad: true, data: ALL_AUTH_ROLES })
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
    const { input, setOptions } = initAuthUserGrantedRolesFieldAction()
    const store = mockMultipleBoardValueStore(input.input)

    setOptions(ALL_AUTH_ROLES)

    return { action: input, store }
}
