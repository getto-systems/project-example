import { mockLocationDetecter } from "../../../../../../../ui/vendor/getto-application/location/mock"

import { detectResetToken } from "./core"

import { ResetPasswordLocationDetecter } from "../method"

export function mockResetPasswordLocationDetecter(currentURL: URL): ResetPasswordLocationDetecter {
    return mockLocationDetecter(currentURL, detectResetToken)
}
