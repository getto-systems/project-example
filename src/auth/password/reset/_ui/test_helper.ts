import { ResetSessionID } from "./data"

export function markResetSessionID(sessionID: string): ResetSessionID {
    return sessionID as ResetSessionID
}
