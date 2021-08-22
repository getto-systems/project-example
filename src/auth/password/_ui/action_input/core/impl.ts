import { initInputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/impl"
import { initValidateBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/impl"

import { checkPasswordCharacter } from "../../check_character/method"

import { InputPasswordAction } from "./action"

import { passwordBoardConverter } from "../../convert"

import { emptyBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export function initInputPasswordAction(): InputPasswordAction {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        converter: () => passwordBoardConverter(store.get()),
    })

    const clear = () => {
        store.set(emptyBoardValue)
        // TODO validate.clear() にしたい
        input.publisher.post()
    }
    const checkCharacter = () => checkPasswordCharacter(store.get())

    subscriber.subscribe(() => {
        validate.check()
    })

    return {
        input,
        validate,
        clear,
        checkCharacter,
        terminate: () => {
            subscriber.terminate()
            validate.terminate()
        },
    }
}
