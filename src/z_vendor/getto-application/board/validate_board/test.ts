import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../action/test_helper"

import { initValidateBoardAction } from "./action"

test("validate; all valid state; clear", async () => {
    const { action, checker } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        checker.update("name", { type: "validated", result: { valid: true } })
        checker.update("description", { type: "validated", result: { valid: true } })
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
        checker.update("name", { type: "validated", result: { valid: false } }) // invalid
        checker.update("description", { type: "validated", result: { valid: true } })
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual(["invalid", "invalid"])
    })
})

test("validate; initial exists", async () => {
    const { action, checker } = standard()

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        checker.update("name", { type: "validated", result: { valid: true } })
        checker.update("description", { type: "initial" })
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual(["initial", "initial"])
    })
})

function standard() {
    const { validate: action, validateChecker: checker } = initValidateBoardAction(
        {
            fields: ["name", "description"],
        },
        {
            convert: () => ({ valid: true, value: { name: "valid-name", value: "valid-value" } }),
        },
    )

    return { action, checker }
}
