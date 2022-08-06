import { test, expect } from "vitest"
import { observeApplicationState } from "../../action/test_helper"

import { initValidateBoardAction } from "./action"

test("validate; all valid state; clear", async () => {
    const { action, checker } = standard()

    expect(
        await observeApplicationState(action.state, async () => {
            checker.update("name", { type: "validated", result: { valid: true } })
            checker.update("description", { type: "validated", result: { valid: true } })
            return action.state.currentState()
        }),
    ).toEqual(["initial", "valid"])

    expect(checker.get()).toEqual({
        valid: true,
        value: { name: "valid-name", value: "valid-value" },
    })

    expect(
        await observeApplicationState(action.state, async () => {
            action.clear()
            return action.state.currentState()
        }),
    ).toEqual(["initial"])
})

test("validate; invalid exists", async () => {
    const { action, checker } = standard()

    expect(
        await observeApplicationState(action.state, async () => {
            checker.update("name", { type: "validated", result: { valid: false } }) // invalid
            checker.update("description", { type: "validated", result: { valid: true } })
            return action.state.currentState()
        }),
    ).toEqual(["invalid", "invalid"])
})

test("validate; initial exists", async () => {
    const { action, checker } = standard()

    expect(
        await observeApplicationState(action.state, async () => {
            checker.update("name", { type: "validated", result: { valid: true } })
            checker.update("description", { type: "initial" })
            return action.state.currentState()
        }),
    ).toEqual(["initial", "initial"])
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
