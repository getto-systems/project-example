import { test, expect } from "vitest"
import { observeApplicationState } from "../../action/test_helper"

import { initValidateBoardFieldAction } from "./action"

import { ValidateBoardFieldResult } from "./data"

test("validate; valid input", async () => {
    // valid input
    const { action } = standard({ valid: true, value: "valid" })

    expect(
        await observeApplicationState(action.state, async () => {
            action.check()
            return action.state.currentState()
        }),
    ).toEqual([{ type: "validated", result: { valid: true, value: "valid" } }])
})

test("validate; invalid input; clear", async () => {
    // invalid input
    const { action } = standard({ valid: false, err: ["empty"] })

    expect(
        await observeApplicationState(action.state, async () => {
            action.check()
            return action.state.currentState()
        }),
    ).toEqual([{ type: "validated", result: { valid: false, err: ["empty"] } }])

    expect(
        await observeApplicationState(action.state, async () => {
            action.clear()
            return action.state.currentState()
        }),
    ).toEqual([{ type: "initial" }])
})

function standard(result: ValidateBoardFieldResult<FieldValue, readonly ValidateError[]>) {
    const action = initValidateBoardFieldAction({
        convert: () => result,
    })

    return { action }
}

type FieldValue = string
type ValidateError = "empty"
