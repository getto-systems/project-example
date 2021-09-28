import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"
import { initValidateBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_validate_field/init"

import { checkPasswordCharacter } from "../check_character/method"

import { InputPasswordAction } from "./action"

import { passwordBoardConverter } from "../input/convert"

import { BoardFieldChecker } from "../../../../../ui/vendor/getto-application/board/validate_field/infra"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { Password, ValidatePasswordError } from "../input/data"

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
