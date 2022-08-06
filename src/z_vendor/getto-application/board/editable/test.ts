import { test, expect } from "vitest"
import { observeApplicationState } from "../../action/test_helper"
import { initEditableBoardAction } from "./action"

test("open / close", async () => {
    const { action } = standard()

    expect(
        await observeApplicationState(action.state, async () => {
            action.open()
            action.close()
            return action.state.currentState()
        }),
    ).toEqual([{ isEditable: true }, { isEditable: false }])
})

function standard() {
    return { action: initEditableBoardAction() }
}
