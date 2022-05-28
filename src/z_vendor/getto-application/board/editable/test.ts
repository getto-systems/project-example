import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../action/test_helper"
import { initEditableBoardAction } from "./action"

test("open / close", async () => {
    const { action } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        action.open()
        action.close()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ isEditable: true }, { isEditable: false }])
    })
})

function standard() {
    return { action: initEditableBoardAction() }
}
