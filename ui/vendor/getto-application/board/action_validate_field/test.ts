import { setupActionTestRunner } from "../../action/test_helper"

import { initValidateBoardFieldAction } from "./init"

import { ValidateBoardFieldAction } from "./action"

import { ConvertBoardFieldResult } from "../validate_field/data"

describe("ValidateBoardField", () => {
    test("validate; valid input", async () => {
        // valid input
        const { action } = standard({ valid: true, value: "valid" })

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.check()).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
            expect(action.get()).toEqual({ valid: true, value: "valid" })
        })
    })

    test("validate; invalid input; clear", async () => {
        // invalid input
        const { action } = standard({ valid: false, err: ["empty"] })

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.check()).then((stack) => {
            expect(stack).toEqual([{ valid: false, err: ["empty"] }])
            expect(action.get()).toEqual({ valid: false, err: ["empty"] })
        })
        await runner(async () => {
            action.clear()
            return action.currentState()
        }).then((stack) => {
            expect(stack).toEqual([{ valid: true }])
        })
    })
})

function standard(result: ConvertBoardFieldResult<FieldValue, ValidateError>) {
    const action: ValidateBoardFieldAction<FieldValue, ValidateError> =
        initValidateBoardFieldAction({ converter: () => result })

    return { action }
}

type FieldValue = string
type ValidateError = "empty"
