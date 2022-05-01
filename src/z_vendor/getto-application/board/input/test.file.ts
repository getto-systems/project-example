import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../action/test_helper"

import { FileStore, SelectFileResult } from "./infra"

import { initSelectFileAction } from "./action"

test("get; store connected", async () => {
    const { source_store, input, store, subscriber } = standard()

    input.connector.connect(source_store)

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            subscriber.subscribe(() => handler(store.get()))
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        input.publisher.post()
    }).then((stack) => {
        expect(stack).toEqual([{ found: true, file: "file" }])
    })
})

test("get; store not connected", async () => {
    const { input, store, subscriber } = standard()

    // store not connected
    //input.connector.connect(source_store)

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            subscriber.subscribe(() => handler(store.get()))
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        input.publisher.post()
    }).then((stack) => {
        expect(stack).toEqual([{ found: false }])
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
        input.publisher.post()
    }).then((stack) => {
        expect(stack).toEqual([])
    })
})

function standard() {
    return {
        // テストでは File として扱わないので as File で無理やり File にしても大丈夫
        source_store: fileStore({ found: true, file: "file" as unknown as File }),
        ...initSelectFileAction(),
    }
}

function fileStore(result: SelectFileResult): FileStore {
    return {
        get: () => result,
    }
}
