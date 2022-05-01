import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../action/test_helper"

import { initValidateBoardFieldAction } from "./action"

import { ValidateBoardFieldResult } from "./data"

test("validate; valid input", async () => {
    // valid input
    const { action } = standard({ valid: true, value: "valid" })

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        action.check()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "validated", result: { valid: true, value: "valid" } }])
    })
})

test("validate; invalid input; clear", async () => {
    // invalid input
    const { action } = standard({ valid: false, err: ["empty"] })

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        action.check()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "validated", result: { valid: false, err: ["empty"] } }])
    })
    await runner(async () => {
        action.clear()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "initial" }])
    })
})

function standard(result: ValidateBoardFieldResult<FieldValue, readonly ValidateError[]>) {
    const action = initValidateBoardFieldAction({
        convert: () => result,
    })

    return { action }
}

type FieldValue = string
type ValidateError = "empty"
