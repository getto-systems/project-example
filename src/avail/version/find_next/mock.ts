import { FindNextVersionDetecter } from "./method"

import { detectApplicationTargetPath } from "./converter"

export function mockFindNextVersionLocationDetecter(
    currentURL: URL,
    version: string,
): FindNextVersionDetecter {
    return () => detectApplicationTargetPath(currentURL, version)
}
