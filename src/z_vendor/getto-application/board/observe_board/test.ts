import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../action/test_helper"

import { initObserveBoardAction } from "./action"

test("observe", async () => {
    const { action, checker } = standard()

    const runner = setupActionTestRunner(action.state)

    await runner(async () => {
        checker.update("name", false)
        checker.update("description", false)
        checker.update("name", true)
        checker.update("description", true)
        checker.update("name", false)
        checker.update("description", false)
        return action.state.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            { hasChanged: false },
            { hasChanged: false },
            { hasChanged: true },
            { hasChanged: true },
            { hasChanged: true },
            { hasChanged: false },
        ])
    })
})

function standard() {
    const { observe: action, observeChecker: checker } = initObserveBoardAction({
        fields: ["name", "description"],
    })

    return { action, checker }
}
