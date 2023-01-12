import { ConvertLocationResult } from "../../../../../common/util/location/data"
import { SignNav, signNavKey } from "../../../../sign/nav/data"
import { ResetToken } from "./data"

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
