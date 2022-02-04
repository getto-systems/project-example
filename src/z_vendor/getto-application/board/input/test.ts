import { setupActionTestRunner } from "../../action/test_helper"

import { mockBoardValueStore } from "./init/mock"
import { markBoardValue } from "../kernel/mock"

import { initInputBoardAction } from "./action"

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
