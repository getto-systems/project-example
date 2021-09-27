import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { BoardFieldValueConverter } from "../../../../../ui/vendor/getto-application/board/validate_field/infra"
import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"
import { SignNav, signNavKey } from "../../../sign/nav/data"
import { Password, ResetToken, ValidatePasswordError } from "./data"

// password には技術的な制限はないが、使用可能な最大文字数は定義しておく
// api の設定と同期すること
export const PASSWORD_MAX_LENGTH = 100

type Converter = BoardFieldValueConverter<Password, BoardValue, ValidatePasswordError>
export const passwordBoardConverter: Converter = (value) => {
    if (value.length === 0) {
        return { valid: false, err: EMPTY }
    }
    if (value.length > PASSWORD_MAX_LENGTH) {
        return { valid: false, err: TOO_LONG }
    }
    return { valid: true, value: markPassword(value) }
}

const EMPTY: ValidatePasswordError[] = [{ type: "empty" }]
const TOO_LONG: ValidatePasswordError[] = [{ type: "too-long", maxLength: PASSWORD_MAX_LENGTH }]

function markPassword(password: string): Password {
    return password as Password
}

export function detectResetToken(currentURL: URL): ConvertLocationResult<ResetToken> {
    const resetToken = currentURL.searchParams.get(signNavKey(SignNav.passwordResetToken))
    if (resetToken === null) {
        return { valid: false }
    }
    if (resetToken.length === 0) {
        return { valid: false }
    }
    return { valid: true, value: markResetToken(resetToken) }
}

function markResetToken(resetToken: string): ResetToken {
    return resetToken as ResetToken
}
