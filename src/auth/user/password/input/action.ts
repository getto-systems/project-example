import { passwordBoardConverter } from "./convert"

import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../z_vendor/getto-application/board/validate_field/action"
import { ApplicationAction } from "../../../../z_vendor/getto-application/action/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"
import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { BoardFieldChecker } from "../../../../z_vendor/getto-application/board/validate_field/infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { Password, PasswordCharacterState, ValidatePasswordError } from "./data"
import {
    BoardValue,
    emptyBoardValue,
} from "../../../../z_vendor/getto-application/board/kernel/data"

export interface InputPasswordAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidatePasswordAction
    readonly observe: ObserveBoardFieldAction

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

    const observe = initObserveBoardFieldAction({
        observer: initBoardFieldObserver({ current: () => store.get() }),
    })

    subscriber.subscribe(() => {
        checker.check()
        observe.check()
    })

    return {
        input: {
            input,
            validate,
            observe,
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
