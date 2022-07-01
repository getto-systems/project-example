import { test, expect } from "vitest"
import { ALL_AUTH_ROLES } from "../../../../../x_content/role"
import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initAuthUserGrantedRolesFilterAction } from "./action"

test("observe; has changed", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.observe)

    await runner(async () => {
        store.set(["auth-user"])
        return action.observe.state.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: true }])
    })
})

test("pin", async () => {
    const { store, pin } = standard()

    store.set(["auth-user"])

    expect(pin()).toEqual(["auth-user"])
})

test("options", async () => {
    const { action } = standard()

    expect(action.options()).toEqual({ type: "loaded", data: ALL_AUTH_ROLES })
})

test("clear", async () => {
    const { action, store } = standard()

    store.set(["auth-user"])
    action.clear()

    expect(store.get()).toEqual([])
})

function standard() {
    const { input: action, setOptions, pin } = initAuthUserGrantedRolesFilterAction([])
    const store = mockMultipleBoardValueStore(action.input)

    setOptions(ALL_AUTH_ROLES)

    return { action, store, pin }
}
