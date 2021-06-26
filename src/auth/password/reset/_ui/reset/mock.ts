import { ResetPasswordDetecter } from "./method"

import { detectResetToken } from "../../../_ui/convert"

export function mockResetPasswordLocationDetecter(currentURL: URL): ResetPasswordDetecter {
    return () => detectResetToken(currentURL)
}
