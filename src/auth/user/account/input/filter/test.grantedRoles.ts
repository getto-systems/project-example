import { test, expect } from "vitest"
import { ALL_AUTH_PERMISSIONS } from "../../../../../x_content/permission"
import { observeApplicationState } from "../../../../../z_vendor/getto-application/action/test_helper"
import { mockMultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initAuthPermissionGrantedFilterAction } from "./action"

test("observe; has changed", async () => {
    const { action, store } = standard()

    expect(
        await observeApplicationState(action.observe.state, async () => {
            store.set(["auth-user"])
            return action.observe.state.currentState()
        }),
    ).toEqual([{ hasChanged: true }])
})

test("pin", async () => {
    const { store, pin } = standard()

    store.set(["auth-user"])

    expect(pin()).toEqual(["auth-user"])
})

test("clear", async () => {
    const { action, store } = standard()

    store.set(["auth-user"])
    action.clear()

    expect(store.get()).toEqual([])
})

function standard() {
    const { input: action, setOptions, pin } = initAuthPermissionGrantedFilterAction([])
    const store = mockMultipleBoardValueStore(action.input)

    setOptions(ALL_AUTH_PERMISSIONS)

    return { action, store, pin }
}
