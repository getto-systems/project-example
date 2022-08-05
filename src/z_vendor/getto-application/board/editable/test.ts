import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../action/test_helper"
import { initEditableBoardAction } from "./action"

test("open / close", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner(action.state)

    await runner(async () => {
        action.open()
        action.close()
        return action.state.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ isEditable: true }, { isEditable: false }])
    })
})

function standard() {
    return { action: initEditableBoardAction() }
}
