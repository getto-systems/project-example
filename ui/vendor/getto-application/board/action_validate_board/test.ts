import { setupActionTestRunner } from "../../action/test_helper"

import { initValidateBoardAction } from "./core/impl"

describe("ValidateBoard", () => {
    test("validate; all valid state", async () => {
        const { action, handler } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            handler.name({ valid: true })
            handler.description({ valid: true })
            return action.initialState
        }).then((stack) => {
            expect(stack).toEqual(["initial", "valid"])
        })
    })

    test("validate; invalid exists", async () => {
        const { action, handler } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            handler.name({ valid: false, err: ["invalid"] }) // invalid
            handler.description({ valid: true })
            return action.initialState
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
            return action.initialState
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
