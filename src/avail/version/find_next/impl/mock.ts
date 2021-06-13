import { mockDetecter } from "../../../../../ui/vendor/getto-application/location/mock"

import { detectApplicationTargetPath } from "./core"

import { FindNextVersionLocationDetecter } from "../method"

export function mockFindNextVersionLocationDetecter(
    currentURL: URL,
    version: string,
): FindNextVersionLocationDetecter {
    return mockDetecter(currentURL, detectApplicationTargetPath({ version }))
}
