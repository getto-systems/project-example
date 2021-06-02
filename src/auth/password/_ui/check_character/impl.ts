import { CheckPasswordCharacterMethod } from "./method"

export const checkPasswordCharacter: CheckPasswordCharacterMethod = (password) => {
    return {
        multiByte: new TextEncoder().encode(password).byteLength > password.length
    }
}
