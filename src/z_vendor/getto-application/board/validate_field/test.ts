import { setupActionTestRunner } from "../../action/test_helper"

import { initValidateBoardFieldAction } from "./action"

import { ConvertBoardFieldResult } from "./data"

test("validate; valid input", async () => {
    // valid input
    const { action, checker } = standard({ valid: true, value: "valid" })

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        checker.check()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ valid: true }])
    })
})

test("validate; invalid input; clear", async () => {
    // invalid input
    const { action, checker } = standard({ valid: false, err: ["empty"] })

    const runner = setupActionTestRunner(action.subscriber)

    await runner(async () => {
        checker.check()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ valid: false, err: ["empty"] }])
    })
    await runner(async () => {
        action.clear()
        return action.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ valid: true }])
    })
})

function standard(result: ConvertBoardFieldResult<FieldValue, ValidateError>) {
    const { validate: action, checker } = initValidateBoardFieldAction({
        converter: () => result,
    })

    return { action, checker }
}

type FieldValue = string
type ValidateError = "empty"
