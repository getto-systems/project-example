import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { PasswordCharacterState } from "../data"

export interface CheckPasswordCharacterMethod {
    (password: BoardValue): PasswordCharacterState
}
