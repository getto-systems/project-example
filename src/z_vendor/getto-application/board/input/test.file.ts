import { test, expect } from "vitest"

import { FileStore, SelectFileResult } from "./infra"

import { initSelectFileAction } from "./action"

test("get; store connected", () => {
    const { source_store, input, store, subscriber } = standard()

    input.connector.connect(source_store)

    const stack: SelectFileResult[] = []
    subscriber.subscribe(() => stack.push(store.get()))
    input.publisher.post()
    expect(stack).toEqual([{ found: true, file: "file" }])
})

test("get; store not connected", () => {
    const { input, store, subscriber } = standard()

    // store not connected
    //input.connector.connect(source_store)

    const stack: SelectFileResult[] = []
    subscriber.subscribe(() => stack.push(store.get()))
    input.publisher.post()
    expect(stack).toEqual([{ found: false }])
})

test("terminate", async () => {
    const { source_store, input, store, subscriber } = standard()

    input.connector.connect(source_store)

    const stack: SelectFileResult[] = []
    subscriber.subscribe(() => stack.push(store.get()))

    subscriber.terminate()
    input.publisher.post()

    expect(stack).toEqual([])
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
