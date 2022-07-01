import { test, expect } from "vitest"

import { initInputBoardAction } from "./action"

import { BoardValueStore } from "./infra"

test("get / set; store connected", () => {
    const { source_store, input, store, subscriber } = standard()

    input.connector.connect(source_store)

    const stack: string[] = []
    subscriber.subscribe(() => stack.push(store.get()))

    source_store.set("value")
    input.publisher.post()

    expect(stack).toEqual(["value"])
})

test("get / set; store not connected", () => {
    const { source_store, input, store, subscriber } = standard()

    // store not connected
    //input.connector.connect(source_store)

    const stack: string[] = []
    subscriber.subscribe(() => stack.push(store.get()))

    source_store.set("value")
    input.publisher.post()

    expect(stack).toEqual([""])
})

test("terminate", () => {
    const { source_store, input, store, subscriber } = standard()

    input.connector.connect(source_store)

    const stack: string[] = []
    subscriber.subscribe(() => stack.push(store.get()))

    subscriber.terminate()
    source_store.set("value")
    input.publisher.post()

    expect(stack).toEqual([])
})

function standard() {
    return { source_store: boardValueStore(), ...initInputBoardAction() }
}

function boardValueStore(): BoardValueStore {
    let storedValue = ""
    return {
        get: () => storedValue,
        set: (value) => {
            storedValue = value
        },
    }
}
