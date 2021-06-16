import { detectResetSessionID } from "../convert"

import { CheckResetTokenSendingStatusDetecter } from "./method"

export function mockCheckResetTokenSendingStatusLocationDetecter(
    currentURL: URL,
): CheckResetTokenSendingStatusDetecter {
    return () => detectResetSessionID(currentURL)
}
