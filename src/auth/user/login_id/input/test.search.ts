import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initFilterLoginIdAction } from "./action"

test("clear", () => {
    const { action, store } = standard()

    store.set("valid")
    action.clear()

    expect(store.get()).toEqual("")
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
        action.input.publisher.post()
        return action.observe.currentState()
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    const { input: action } = initFilterLoginIdAction({ filter: false })
    const store = mockBoardValueStore(action.input)

    return { action, store }
}
