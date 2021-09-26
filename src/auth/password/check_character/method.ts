import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { PasswordCharacterState } from "../input/data"

export interface CheckPasswordCharacterMethod {
    (password: BoardValue): PasswordCharacterState
}

export const checkPasswordCharacter: CheckPasswordCharacterMethod = (password) => {
    return {
        multiByte: new TextEncoder().encode(password).byteLength > password.length
    }
}
