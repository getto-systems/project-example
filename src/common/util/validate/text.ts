import { Validator } from "./infra"

import { ValidateResult, ValidateTextError } from "./data"

export function check_text_empty(text: string): ValidateResult<ValidateTextError> {
    if (text.length === 0) {
        return { valid: false, err: { type: "empty" } }
    }
    return { valid: true }
}
export function check_text_tooLong(maxLength: number): Validator<string, ValidateTextError> {
    return (text) => {
        if (text.length > maxLength) {
            return { valid: false, err: { type: "too-long", maxLength } }
        }
        return { valid: true }
    }
}
export function check_text_invalidEmail(text: string): ValidateResult<ValidateTextError> {
    if (text.length === 0) {
        // empty は check_text_empty で判定する
        return { valid: true }
    }
    if (!text.includes("@")) {
        return { valid: false, err: { type: "invalid-email" } }
    }
    return { valid: true }
}
