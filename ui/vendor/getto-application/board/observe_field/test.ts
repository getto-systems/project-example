import { setupActionTestRunner } from "../../action/test_helper"

import { mockBoardValueStore } from "../input/init/mock"
import { markBoardValue } from "../kernel/mock"

import { initObserveBoardFieldAction } from "./action"
import { initBoardFieldObserver } from "./init/observer"

describe("ObserveBoardField", () => {
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
})

function standard() {
    const store = mockBoardValueStore()
    const observer = initBoardFieldObserver(() => store.get())
    const action = initObserveBoardFieldAction({ observer })

    return { action, observer, store }
}
