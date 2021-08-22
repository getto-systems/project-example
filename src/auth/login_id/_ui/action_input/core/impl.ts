import { initInputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/impl"
import { initValidateBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/impl"

import { InputLoginIDAction } from "./action"

import { loginIDBoardConverter } from "../../convert"
import { emptyBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export function initInputLoginIDAction(): InputLoginIDAction {
    const { input, store, publisher, subscriber } = initInputBoardAction()

    const validate = initValidateBoardFieldAction({
        converter: () => loginIDBoardConverter(store.get()),
    })

    subscriber.subscribe(() => validate.check())

    return {
        input,
        validate,
        clear: () => {
            store.set(emptyBoardValue)
            // TODO validate.clear() にしたい
            publisher.post()
        },
        terminate: () => {
            // TODO subscriber の terminate のテストをしたい
            subscriber.terminate()
            validate.terminate()
        },
    }
}
