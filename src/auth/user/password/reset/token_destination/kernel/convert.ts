import {
    ConvertResetTokenDestinationResult,
    ResetTokenDestination,
    ResetTokenDestinationEmail,
} from "./data"
import { ConvertBoardFieldResult } from "../../../../../../z_vendor/getto-application/board/validate_field/data"
import { converter } from "../../../../../../z_lib/ui/validate/helper"
import {
    check_text_empty,
    check_text_invalidEmail,
    check_text_tooLong,
} from "../../../../../../z_lib/ui/validate/text"
import { ValidateTextError } from "../../../../../../z_lib/ui/validate/data"

// email には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期させること
export const EMAIL_MAX_LENGTH = 255

export function resetTokenDestinationBoardConverter(
    value: Readonly<{ type: string; email: string }>,
): ConvertResetTokenDestinationResult {
    switch (value.type) {
        case "none":
            return { valid: true, value: { type: "none" } }

        case "email":
            return validateEmail(value.email)

        default:
            return { valid: false, err: { type: "type", err: [{ type: "invalid-type" }] } }
    }
}

function validateEmail(value: string): ConvertResetTokenDestinationResult {
    const result = emailConverter(value)
    if (!result.valid) {
        return { valid: false, err: { type: "email", err: result.err } }
    }
    return { valid: true, value: { type: "email", email: result.value } }
}

const emailConverter: {
    (value: string): ConvertBoardFieldResult<
        ResetTokenDestinationEmail,
        readonly ValidateTextError[]
    >
} = converter(
    (value: string) => value as ResetTokenDestinationEmail,
    [
        check_text_empty,
        // email には技術的な制限はないが、使用可能な最大文字数は定義しておく
        // api の設定と同期させること
        check_text_tooLong(255),
        check_text_invalidEmail,
    ],
)

export function restoreResetTokenDestination(
    data: Readonly<{ type: string; email: string }>,
): ResetTokenDestination {
    switch (data.type) {
        case "email":
            return {
                type: "email",
                email: data.email as string as ResetTokenDestinationEmail,
            }

        default:
            return { type: "none" }
    }
}
