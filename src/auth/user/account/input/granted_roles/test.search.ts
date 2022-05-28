import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initFilterGrantedRolesAction } from "./action"

test("observe; has changed", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.observe.subscriber)

    await runner(async () => {
        store.grantedRoles.set(["user"])
        return action.observe.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: true }])
    })
})

test("clear", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.observe.subscriber)

    await runner(async () => {
        store.grantedRoles.set(["user"])
        action.clear()
        return action.observe.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: true }, { hasChanged: false }])
        expect(store.grantedRoles.get()).toEqual([])
    })
})

function standard() {
    const { input: action } = initFilterGrantedRolesAction([])
    const store = {
        grantedRoles: mockMultipleBoardValueStore(action.input),
    }

    return { action, store }
}
