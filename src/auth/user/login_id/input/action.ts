import { loginIDBoardConverter } from "./convert"

import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"
import { initValidateBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_validate_field/init"

import { ApplicationAction } from "../../../../../ui/vendor/getto-application/action/action"
import { InputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"
import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../../ui/vendor/getto-application/board/action_validate_field/action"

import { BoardFieldChecker } from "../../../../../ui/vendor/getto-application/board/validate_field/infra"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { LoginID, ValidateLoginIDError } from "./data"

export interface InputLoginIDAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly validate: ValidateLoginIDAction
    clear(): void
}

export type ValidateLoginIDAction = ValidateBoardFieldAction<ValidateLoginIDError>
export type ValidateLoginIDState = ValidateBoardFieldState<ValidateLoginIDError>

export function initInputLoginIDAction(): Readonly<{
    input: InputLoginIDAction
    checker: BoardFieldChecker<LoginID, ValidateLoginIDError>
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const { validate, checker } = initValidateBoardFieldAction({
        converter: () => loginIDBoardConverter(store.get()),
    })

    subscriber.subscribe(() => checker.check())

    return {
        input: {
            input,
            validate,
            clear: () => {
                store.set(emptyBoardValue)
                validate.clear()
            },
            terminate: () => {
                subscriber.terminate()
                validate.terminate()
            },
        },
        checker,
    }
}
