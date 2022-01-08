import { setupActionTestRunner } from "../../action/test_helper"

import { initObserveBoardAction } from "./action"

describe("ObserveBoard", () => {
    test("observe", async () => {
        const { action, checker } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            checker.update("name", false)
            checker.update("description", false)
            checker.update("name", true)
            checker.update("description", true)
            checker.update("name", false)
            checker.update("description", false)
            return action.currentState()
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
})

function standard() {
    const { observe: action, checker } = initObserveBoardAction({
        fields: ["name", "description"],
    })

    return { action, checker }
}
