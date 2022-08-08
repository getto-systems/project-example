import { test, expect } from "vitest"
import { observeApplicationState } from "../../action/test_helper"

import { initObserveBoardAction } from "./action"

test("observe", async () => {
    const { action, checker } = standard()

    expect(
        await observeApplicationState(action.state, async () => {
            checker.update("name", false)
            checker.update("description", false)
            checker.update("name", true)
            checker.update("description", true)
            checker.update("name", false)
            checker.update("description", false)
            return action.state.currentState()
        }),
    ).toEqual([
        { hasChanged: false },
        { hasChanged: false },
        { hasChanged: true },
        { hasChanged: true },
        { hasChanged: true },
        { hasChanged: false },
    ])
})

function standard() {
    const { observe: action, observeChecker: checker } = initObserveBoardAction({
        fields: ["name", "description"],
    })

    return { action, checker }
}
