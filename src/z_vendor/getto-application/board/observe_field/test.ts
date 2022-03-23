import { setupActionTestRunner } from "../../action/test_helper"

import { mockBoardValueStore } from "../input/test_helper"
import { markBoardValue } from "../kernel/test_helper"

import { initObserveBoardFieldAction } from "./action"
import { isSameMultipleBoardValue } from "./helper"
import { initBoardFieldObserver } from "./init/observer"

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
        store.set(markBoardValue("changed"))
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
    const a = ["a", "b", "c"].map(markBoardValue)
    const b = ["a", "b", "c"].map(markBoardValue)
    expect(isSameMultipleBoardValue(a, b)).toBe(true)
})

test("check same value; some different value", async () => {
    const a = ["a", "b", "c"].map(markBoardValue)
    const b = ["a", "b", "x"].map(markBoardValue)
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

test("check same value; different order", async () => {
    const a = ["a", "b", "c"].map(markBoardValue)
    const b = ["a", "c", "b"].map(markBoardValue)
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

test("check same value; different length", async () => {
    const a = ["a", "b", "c"].map(markBoardValue)
    const b = ["a", "b"].map(markBoardValue)
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

test("check same value; zero length first argument", async () => {
    const a = [].map(markBoardValue)
    const b = ["a", "b"].map(markBoardValue)
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

test("check same value; zero length second argument", async () => {
    const a = ["a", "b"].map(markBoardValue)
    const b = [].map(markBoardValue)
    expect(isSameMultipleBoardValue(a, b)).toBe(false)
})

function standard() {
    const store = mockBoardValueStore()
    const observer = initBoardFieldObserver({
        current: () => store.get(),
        isSame: (a, b) => a === b,
    })
    const action = initObserveBoardFieldAction({ observer })

    return { action, observer, store }
}
