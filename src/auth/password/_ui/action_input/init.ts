import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"
import { initValidateBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_validate_field/init"

import { checkPasswordCharacter } from "../check_character/method"

import { InputPasswordAction } from "./action"

import { passwordBoardConverter } from "../convert"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function initInputPasswordAction(): InputPasswordAction {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        converter: () => passwordBoardConverter(store.get()),
    })

    subscriber.subscribe(() => {
        validate.check()
    })

    return {
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
    }
}
