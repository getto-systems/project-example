import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"
import { SignNav, signNavKey } from "../../../_ui/common/nav/data"
import { ResetSessionID } from "./data"

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

export function convertResetSessionIDRemote(sessionID: string): ResetSessionID {
    // remote からの値は validation チェックなしで受け入れる
    return markResetSessionID(sessionID)
}

function markResetSessionID(sessionID: string): ResetSessionID {
    return sessionID as ResetSessionID
}
