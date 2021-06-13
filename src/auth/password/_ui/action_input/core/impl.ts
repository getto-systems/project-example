import { initInputBoardValueResource } from "../../../../../../ui/vendor/getto-application/board/action_input/impl"
import { initValidateBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/impl"

import { checkPasswordCharacter } from "../../check_character/method"

import { InputPasswordAction } from "./action"

import { passwordBoardConverter } from "../../converter"

export function initInputPasswordAction(): InputPasswordAction {
    const board = initInputBoardValueResource("password")

    const validate = initValidateBoardFieldAction({
        converter: () => passwordBoardConverter(board.input.get()),
    })

    const clear = () => board.input.clear()
    const checkCharacter = () => checkPasswordCharacter(board.input.get())

    board.input.subscribeInputEvent(() => {
        validate.check()
    })

    return {
        board,
        validate,
        clear,
        checkCharacter,
        terminate: () => {
            board.input.terminate()
            validate.terminate()
        },
    }
}
