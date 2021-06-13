import { mockDetecter } from "../../../../../../ui/vendor/getto-application/location/mock"

import { ResetPasswordDetecter } from "./method"

import { detectResetToken } from "../converter"

export function mockResetPasswordLocationDetecter(currentURL: URL): ResetPasswordDetecter {
    return mockDetecter(currentURL, detectResetToken)
}
