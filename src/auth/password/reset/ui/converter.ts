import { LocationConverter } from "../../../../../ui/vendor/getto-application/location/infra"

import { ResetSessionID, ResetToken } from "./data"

type ResetTokenConverter = LocationConverter<ResetToken, string | null>
export const resetTokenLocationConverter: ResetTokenConverter = (resetToken) => {
    if (resetToken === null) {
        return { valid: false }
    }
    if (resetToken.length === 0) {
        return { valid: false }
    }
    return { valid: true, value: markResetToken(resetToken) }
}

type ResetSessionIDConverter = LocationConverter<ResetSessionID, string | null>
export const resetSessionIDLocationConverter: ResetSessionIDConverter = (sessionID) => {
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
