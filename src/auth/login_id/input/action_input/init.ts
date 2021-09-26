import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"
import { initValidateBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_validate_field/init"

import { InputLoginIDAction } from "./action"

import { loginIDBoardConverter } from "../convert"

import { BoardFieldChecker } from "../../../../../ui/vendor/getto-application/board/validate_field/infra"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { LoginID, ValidateLoginIDError } from "../data"

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
