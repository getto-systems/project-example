import { initInputBoardValueResource } from "../../../../../../ui/vendor/getto-application/board/action_input/impl"
import { initValidateBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/impl"

import { InputLoginIDAction } from "./action"

import { loginIDBoardConverter } from "../../converter"

export function initInputLoginIDAction(): InputLoginIDAction {
    const board = initInputBoardValueResource("text")

    const validate = initValidateBoardFieldAction({
        converter: () => loginIDBoardConverter(board.input.get()),
    })

    board.input.subscribeInputEvent(() => validate.check())

    return {
        board,
        validate,
        clear: () => board.input.clear(),
        terminate: () => {
            board.input.terminate()
            validate.terminate()
        },
    }
}
