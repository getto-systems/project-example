import { setupActionTestRunner } from "../../action/test_helper"

import { mockBoardValueStore } from "./mock"
import { markBoardValue } from "../kernel/mock"

import { initInputBoardValueAction } from "./core/impl"

describe("InputBoardValue", () => {
    test("get / set / clear; store linked", async () => {
        const { action, store } = standard()

        action.storeLinker.link(store)

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.subscribeInputEvent(() => handler(action.get()))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            action.set(markBoardValue("value"))
        }).then((stack) => {
            expect(stack).toEqual(["value"])
        })
        await runner(async () => {
            action.clear()
        }).then((stack) => {
            expect(stack).toEqual([""])
        })
    })

    test("set; no store linked", async () => {
        const { action } = standard()

        // no linked store

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.subscribeInputEvent(() => handler(action.get()))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            action.set(markBoardValue("value"))
        }).then((stack) => {
            expect(stack).toEqual([""])
        })
    })

    test("terminate", async () => {
        const { action, store } = standard()

        action.storeLinker.link(store)

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                action.subscribeInputEvent(() => handler(action.get()))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            action.terminate()
            action.set(markBoardValue("value"))
        }).then((stack) => {
            // no event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const action = initInputBoardValueAction()
    const store = mockBoardValueStore()

    return { action, store }
}
