import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../z_vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"

import { initInputLoginIDAction } from "./action"

describe("InputLoginID", () => {
    test("validate; valid input", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.set(markBoardValue("valid"))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
        })
    })

    test("validate; invalid : empty", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.set(markBoardValue(""))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: [{ type: "empty" }] }])
        })
    })

    test("validate; invalid : too-long", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.set(markBoardValue("a".repeat(100 + 1)))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: [{ type: "too-long", maxLength: 100 }] }])
        })
    })

    test("validate; valid : just max-length", async () => {
        const { action, store } = standard()

        const runner = setupActionTestRunner(action.validate.subscriber)

        await runner(async () => {
            store.set(markBoardValue("a".repeat(100)))
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
        })
    })

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
                action.validate.subscriber.subscribe(handler)
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            action.terminate()
            action.input.publisher.post()
            return action.validate.currentState()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const { input: action } = initInputLoginIDAction()
    const store = mockBoardValueStore()
    action.input.connector.connect(store)

    return { action, store }
}
