import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/test_helper"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initSearchLoginIdAction } from "./action"

describe("SearchLoginId", () => {
    test("clear", () => {
        const { action, store } = standard()

        store.set(markBoardValue("valid"))
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
})

function standard() {
    const { input: action } = initSearchLoginIdAction({ search: false })
    const store = mockBoardValueStore()
    action.input.connector.connect(store)

    return { action, store }
}
