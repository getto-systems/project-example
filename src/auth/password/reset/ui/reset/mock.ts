import { ResetPasswordDetecter } from "./method"

import { detectResetToken } from "../converter"

export function mockResetPasswordLocationDetecter(currentURL: URL): ResetPasswordDetecter {
    return () => detectResetToken(currentURL)
}
