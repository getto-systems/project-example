import { ConvertLocationResult } from "../../../../z_lib/ui/location/data"
import { SignNav, signNavKey } from "../../../sign/nav/data"
import { ConvertPasswordResult, Password, ResetToken } from "./data"
import { converter } from "../../../../z_lib/ui/validate/helper"
import { check_text_empty, check_text_tooLong } from "../../../../z_lib/ui/validate/text"

export const PASSWORD_MAX_LENGTH = 100

export const passwordBoardConverter: { (value: string): ConvertPasswordResult } = converter(
    markPassword,
    [
        check_text_empty,
        // password には意味的な制限はないが、使用可能な最大文字数は定義しておく
        // api の設定と同期すること
        check_text_tooLong(100),
    ],
)

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
