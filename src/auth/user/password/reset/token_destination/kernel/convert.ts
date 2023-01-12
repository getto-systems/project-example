import {
    ValidateResetTokenDestinationResult,
    ResetTokenDestination,
    ResetTokenDestinationEmail,
} from "./data"
import { ValidateBoardFieldResult } from "../../../../../../z_vendor/getto-application/board/validate_field/data"
import { converter } from "../../../../../../common/util/validate/helper"
import {
    check_text_empty,
    check_text_invalidEmail,
    check_text_tooLong,
} from "../../../../../../common/util/validate/text"
import { ValidateTextError } from "../../../../../../common/util/validate/data"

export function resetTokenDestinationBoardConverter(
    value: Readonly<{ type: string; email: string }>,
): ValidateResetTokenDestinationResult {
    switch (value.type) {
        case "none":
            return { valid: true, value: { type: "none" } }

        case "email":
            return validateEmail(value.email)

        default:
            return { valid: false, err: { type: "type", err: [{ type: "invalid-type" }] } }
    }
}

function validateEmail(value: string): ValidateResetTokenDestinationResult {
    const result = emailConverter(value)
    if (!result.valid) {
        return { valid: false, err: { type: "email", err: result.err } }
    }
    return { valid: true, value: { type: "email", email: result.value } }
}

const emailConverter: {
    (value: string): ValidateBoardFieldResult<
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
