import { setupActionTestRunner } from "../../../../../../ui/vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../../../ui/vendor/getto-application/board/input/init/mock"

import { initSearchOffsetAction } from "./init"

describe("SearchOffset", () => {
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
    const { input: action } = initSearchOffsetAction(markBoardValue(""))
    const store = mockBoardValueStore()
    action.input.connector.connect(store)

    return { action, store }
}
