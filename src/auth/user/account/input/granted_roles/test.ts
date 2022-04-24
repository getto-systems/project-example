import { setupActionTestRunner } from "../../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../../z_vendor/getto-application/board/input/test_helper"

import { initInputGrantedAuthRolesAction } from "./action"

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

test("terminate", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            action.observe.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        action.terminate()
        return action.observe.currentState()
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    const { input: action } = initInputGrantedAuthRolesAction()
    const store = {
        grantedRoles: mockMultipleBoardValueStore(action.input),
    }

    return { action, store }
}
