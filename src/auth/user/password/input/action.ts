import { passwordBoardConverter } from "./convert"

import {
    initInputBoardAction,
    InputBoardAction,
} from "../../../../z_vendor/getto-application/board/input/action"
import {
    initValidateBoardFieldAction,
    ValidateBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/validate_field/action"
import { ApplicationAction } from "../../../../z_vendor/getto-application/action/action"
import {
    initObserveBoardFieldAction,
    ObserveBoardFieldAction,
} from "../../../../z_vendor/getto-application/board/observe_field/action"
import { initBoardFieldObserver } from "../../../../z_vendor/getto-application/board/observe_field/init/observer"

import { BoardFieldChecker } from "../../../../z_vendor/getto-application/board/validate_field/infra"
import { BoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

import { Password, PasswordCharacterState } from "./data"
import { ValidateTextError } from "../../../../z_lib/ui/validate/data"

export interface InputPasswordAction extends ApplicationAction {
    readonly input: InputBoardAction<BoardValueStore>
    readonly validate: ValidateBoardFieldAction<readonly ValidateTextError[]>
    readonly observe: ObserveBoardFieldAction

    clear(): void
    checkCharacter(): PasswordCharacterState
}

export function initInputPasswordAction(): Readonly<{
    input: InputPasswordAction
    checker: BoardFieldChecker<Password, readonly ValidateTextError[]>
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
                store.set("")
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

function checkPasswordCharacter(password: string): PasswordCharacterState {
    return {
        multiByte: new TextEncoder().encode(password).byteLength > password.length,
    }
}
