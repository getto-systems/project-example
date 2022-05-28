import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initInputGrantedAuthRolesAction } from "./action"

test("validate; valid input", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.validate.subscriber)

    await runner(async () => {
        store.grantedRoles.set(["auth-user"])
        return action.validate.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "validated", result: { valid: true, value: ["auth-user"] } }])
    })
})

test("observe; has changed", async () => {
    const { action, store } = standard()

    const runner = setupActionTestRunner(action.observe.subscriber)

    await runner(async () => {
        store.grantedRoles.set(["auth-user"])
        return action.observe.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: true }])
    })
})

function standard() {
    const action = initInputGrantedAuthRolesAction()
    const store = {
        grantedRoles: mockMultipleBoardValueStore(action.input),
    }

    return { action, store }
}
