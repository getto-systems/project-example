import { BoardValue } from "../../../../../../z_vendor/getto-application/board/kernel/data"
import { ResetTokenDestination, ResetTokenDestinationEmail } from "../kernel/data"
import { ValidateResetTokenDestinationError } from "./data"
import { ConvertBoardFieldResult } from "../../../../../../z_vendor/getto-application/board/validate_field/data"

// email には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期させること
export const EMAIL_MAX_LENGTH = 255

export function resetTokenDestinationBoardConverter(
    value: Readonly<{ type: BoardValue; email: BoardValue }>,
): ConvertBoardFieldResult<ResetTokenDestination, ValidateResetTokenDestinationError> {
    switch (value.type) {
        case "none":
            return { valid: true, value: { type: "none" } }

        case "email":
            if (value.email.length === 0) {
                return { valid: false, err: [{ type: "empty-email" }] }
            }
            if (!value.email.includes("@")) {
                return { valid: false, err: [{ type: "invalid-email" }] }
            }
            if (value.email.length > EMAIL_MAX_LENGTH) {
                return {
                    valid: false,
                    err: [{ type: "too-long-email", maxLength: EMAIL_MAX_LENGTH }],
                }
            }
            return {
                valid: true,
                value: {
                    type: "email",
                    email: value.email as string as ResetTokenDestinationEmail,
                },
            }

        default:
            return { valid: false, err: [{ type: "invalid-type" }] }
    }
}

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
