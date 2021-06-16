import { ResetPasswordDetecter } from "./method"

import { detectResetToken } from "../convert"

export function mockResetPasswordLocationDetecter(currentURL: URL): ResetPasswordDetecter {
    return () => detectResetToken(currentURL)
}
