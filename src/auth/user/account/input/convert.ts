import { BoardFieldValueConverter } from "../../../../z_vendor/getto-application/board/validate_field/infra"

import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import {
    GrantedRole,
    isGrantedRole,
    ResetTokenDestination,
    ResetTokenDestinationEmail,
    ValidateResetTokenDestinationError,
} from "./data"

export function toGrantedRoles(roles: readonly string[]): readonly GrantedRole[] {
    const grantedRoles: GrantedRole[] = []
    roles.forEach((role) => {
        if (isGrantedRole(role)) {
            grantedRoles.push(role)
        }
    })
    return grantedRoles
}

// email には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期させること
export const EMAIL_MAX_LENGTH = 100

type Converter = BoardFieldValueConverter<
    ResetTokenDestination,
    BoardValue,
    ValidateResetTokenDestinationError
>
export const resetTokenDestinationBoardConverter: Converter = (value) => {
    if (value.length === 0) {
        return { valid: false, err: EMPTY }
    }
    if (!value.includes("@")) {
        return { valid: false, err: INVALID_EMAIL }
    }
    if (value.length > EMAIL_MAX_LENGTH) {
        return { valid: false, err: TOO_LONG }
    }
    return { valid: true, value: markEmail(value) }
}

const EMPTY: readonly ValidateResetTokenDestinationError[] = [{ type: "empty-email" }]
const INVALID_EMAIL: readonly ValidateResetTokenDestinationError[] = [{ type: "invalid-email" }]
const TOO_LONG: readonly ValidateResetTokenDestinationError[] = [
    { type: "too-long-email", maxLength: EMAIL_MAX_LENGTH },
]

function markEmail(email: string): ResetTokenDestination {
    return { type: "email", email: email as ResetTokenDestinationEmail }
}
