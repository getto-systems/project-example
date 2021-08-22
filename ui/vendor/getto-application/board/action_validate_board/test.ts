import { setupActionTestRunner } from "../../action/test_helper"

import { initValidateBoardAction } from "./init"

describe("ValidateBoard", () => {
    test("validate; all valid state; clear", async () => {
        const { action, handler } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            handler.name({ valid: true })
            handler.description({ valid: true })
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
        const { action, handler } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            handler.name({ valid: false, err: ["invalid"] }) // invalid
            handler.description({ valid: true })
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual(["invalid", "invalid"])
        })
    })

    test("validate; initial exists", async () => {
        const { action, handler } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            handler.name({ valid: true })
            // description: initial state
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual(["initial"])
        })
    })

    test("get", () => {
        const { action } = standard()

        expect(action.get()).toEqual({
            valid: true,
            value: { name: "valid-name", value: "valid-value" },
        })
    })
})

function standard() {
    const action = initValidateBoardAction({
        fields: ["name", "description"],
        converter: () => ({ valid: true, value: { name: "valid-name", value: "valid-value" } }),
    })

    const handler = {
        name: action.updateValidateState("name"),
        description: action.updateValidateState("description"),
    }

    return { action, handler }
}
