import { setupActionTestRunner } from "../../action/test_helper"

import { initObserveBoardFieldAction } from "./action"
import { isSameMultipleBoardValue } from "./helper"
import { initBoardFieldObserver } from "./init/observer"

import { BoardValueStore } from "../input/infra"

test("observe; no change", async () => {
    const { action, observer } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        observer.pin()
        action.check()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: false }])
    })
})

test("observe; has changed", async () => {
    const { action, observer, store } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        observer.pin()
        store.set("changed")
        action.check()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: true }])
    })
})

test("observe; initial", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        action.check()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ hasChanged: false }])
    })
})

test("check same value; all same value", async () => {
    const a = ["a", "b", "c"]
    const b = ["a", "b", "c"]
    expect(isSameMultipleBoardValue(a, b)).toBe(true)
})

test("check same value; some different value", async () => {
    const a = ["a", "b", "c"]
    const b = ["a", "b", "x"]
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

test("check same value; different order", async () => {
    const a = ["a", "b", "c"]
    const b = ["a", "c", "b"]
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

test("check same value; different length", async () => {
    const a = ["a", "b", "c"]
    const b = ["a", "b"]
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

test("check same value; zero length first argument", async () => {
    expect(isSameMultipleBoardValue([], ["a", "b"])).toBe(false)
})

test("check same value; zero length second argument", async () => {
    expect(isSameMultipleBoardValue(["a", "b"], [])).toBe(false)
})

function standard() {
    const store = boardValueStore()
    const observer = initBoardFieldObserver({
        current: () => store.get(),
        isSame: (a, b) => a === b,
    })
    const action = initObserveBoardFieldAction({ observer })

    return { action, observer, store }
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
