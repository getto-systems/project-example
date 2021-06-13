import { detectMenuTargetPath } from "./converter"

import { LoadMenuDetecter } from "./method"

export function mockLoadMenuLocationDetecter(currentURL: URL, version: string): LoadMenuDetecter {
    return () => detectMenuTargetPath(currentURL, version)
}
