import { setupActionTestRunner } from "../../action/test_helper"

import { initValidateBoardAction } from "./action"

describe("ValidateBoard", () => {
    test("validate; all valid state; clear", async () => {
        const { action, checker } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            checker.update("name", true)
            checker.update("description", true)
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual(["initial", "valid"])
        })
        await runner(async () => {
            action.clear()
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual(["initial"])
        })
    })

    test("validate; invalid exists", async () => {
        const { action, checker } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            checker.update("name", false) // invalid
            checker.update("description", true)
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual(["invalid", "invalid"])
        })
    })

    test("validate; initial exists", async () => {
        const { action, checker } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            checker.update("name", true)
            // description: initial state
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual(["initial"])
        })
    })
})

function standard() {
    const { validate: action, checker } = initValidateBoardAction(
        {
            fields: ["name", "description"],
        },
        {
            converter: () => ({ valid: true, value: { name: "valid-name", value: "valid-value" } }),
        },
    )

    return { action, checker }
}
