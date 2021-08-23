import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"
import { initValidateBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_validate_field/init"

import { InputLoginIDAction } from "./action"

import { loginIDBoardConverter } from "../convert"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function initInputLoginIDAction(): InputLoginIDAction {
    const { input, store, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        converter: () => loginIDBoardConverter(store.get()),
    })

    subscriber.subscribe(() => validate.check())

    return {
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
    }
}