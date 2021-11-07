import { FindNextVersionDetecter } from "./infra"

import { detectApplicationTargetPath } from "./convert"

export function mockFindNextVersionLocationDetecter(
    currentURL: URL,
    version: string,
): FindNextVersionDetecter {
    return () => detectApplicationTargetPath(currentURL, version)
}
