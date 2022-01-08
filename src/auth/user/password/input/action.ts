import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"
import { initValidateBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_validate_field/init"
import { passwordBoardConverter } from "./convert"

import { BoardFieldChecker } from "../../../../../ui/vendor/getto-application/board/validate_field/infra"

import { InputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"
import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../../ui/vendor/getto-application/board/action_validate_field/action"
import { ApplicationAction } from "../../../../../ui/vendor/getto-application/action/action"

import { Password, PasswordCharacterState, ValidatePasswordError } from "./data"
import {
    BoardValue,
    emptyBoardValue,
} from "../../../../../ui/vendor/getto-application/board/kernel/data"

export interface InputPasswordAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly validate: ValidatePasswordAction

    clear(): void
    checkCharacter(): PasswordCharacterState
}

export type ValidatePasswordAction = ValidateBoardFieldAction<ValidatePasswordError>
export type ValidatePasswordState = ValidateBoardFieldState<ValidatePasswordError>

export function initInputPasswordAction(): Readonly<{
    input: InputPasswordAction
    checker: BoardFieldChecker<Password, ValidatePasswordError>
}> {
    const { input, store, subscriber } = initInputBoardAction()

    const { validate, checker } = initValidateBoardFieldAction({
        converter: () => passwordBoardConverter(store.get()),
    })

    subscriber.subscribe(() => {
        checker.check()
    })

    return {
        input: {
            input,
            validate,
            clear: () => {
                store.set(emptyBoardValue)
                validate.clear()
            },
            checkCharacter: () => checkPasswordCharacter(store.get()),
            terminate: () => {
                subscriber.terminate()
                validate.terminate()
            },
        },
        checker,
    }
}

function checkPasswordCharacter(password: BoardValue): PasswordCharacterState {
    return {
        multiByte: new TextEncoder().encode(password).byteLength > password.length,
    }
}
