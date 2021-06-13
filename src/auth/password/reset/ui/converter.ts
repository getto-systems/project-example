import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"
import { SignNav, signNavKey } from "../../../_ui/common/nav/data"
import { ResetSessionID, ResetToken } from "./data"

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

export function detectResetSessionID(currentURL: URL): ConvertLocationResult<ResetSessionID> {
    const sessionID = currentURL.searchParams.get(signNavKey(SignNav.passwordResetSessionID))
    if (sessionID === null) {
        return { valid: false }
    }
    if (sessionID.length === 0) {
        return { valid: false }
    }
    return { valid: true, value: markResetSessionID(sessionID) }
}

export function resetSessionIDRemoteConverter(sessionID: string): ResetSessionID {
    // remote からの値は validation チェックなしで受け入れる
    return markResetSessionID(sessionID)
}

function markResetSessionID(sessionID: string): ResetSessionID {
    return sessionID as ResetSessionID
}
function markResetToken(resetToken: string): ResetToken {
    return resetToken as ResetToken
}
