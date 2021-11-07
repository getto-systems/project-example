import { ResetPasswordDetecter } from "./infra"

import { detectResetToken } from "../../input/convert"

export function mockResetPasswordLocationDetecter(currentURL: URL): ResetPasswordDetecter {
    return () => detectResetToken(currentURL)
}
