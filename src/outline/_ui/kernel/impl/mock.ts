import { mockLocationDetecter } from "../../../../../ui/vendor/getto-application/location/mock"

import { detectMenuTargetPath } from "./detecter"

import { LoadMenuLocationDetecter } from "../method"

export function mockLoadMenuLocationDetecter(
    currentURL: URL,
    version: string,
): LoadMenuLocationDetecter {
    return mockLocationDetecter(currentURL, detectMenuTargetPath({ version }))
}
