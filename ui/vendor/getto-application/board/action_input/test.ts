import { setupActionTestRunner } from "../../action/test_helper"

import { mockBoardValueStore } from "./mock"
import { markBoardValue } from "../kernel/mock"

import { initInputBoardValueAction } from "./core/impl"
import { initInputBoardAction } from "./impl"

describe("InputBoard", () => {
    test("get / set; store connected", async () => {
        const { source_store, input, store, subscriber } = standard()

        input.connector.connect(source_store)

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                subscriber.subscribe(() => handler(store.get()))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            source_store.set(markBoardValue("value"))
            input.publisher.post()
        }).then((stack) => {
            expect(stack).toEqual(["value"])
        })
    })

    test("get / set; store not connected", async () => {
        const { source_store, input, store, subscriber } = standard()

        // store not connected
        //input.connector.connect(source_store)

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                subscriber.subscribe(() => handler(store.get()))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            source_store.set(markBoardValue("value"))
            input.publisher.post()
        }).then((stack) => {
            expect(stack).toEqual([""])
        })
    })

    test("terminate", async () => {
        const { source_store, input, store, subscriber } = standard()

        input.connector.connect(source_store)

        const runner = setupActionTestRunner({
            subscribe: (handler) => {
                subscriber.subscribe(() => handler(store.get()))
            },
            unsubscribe: () => null,
        })

        await runner(async () => {
            subscriber.terminate()
            source_store.set(markBoardValue("value"))
            input.publisher.post()
        }).then((stack) => {
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    return { source_store: mockBoardValueStore(), ...initInputBoardAction() }
}

// TODO 削除予定
describe("InputBoardValue", () => {
    test("get / set / clear; store linked", async () => {
        const { action, store } = standard_legacy()

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
        const { action } = standard_legacy()

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
        const { action, store } = standard_legacy()

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

function standard_legacy() {
    const action = initInputBoardValueAction()
    const store = mockBoardValueStore()

    return { action, store }
}
