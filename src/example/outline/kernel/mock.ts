import { LoadMenuDetecter } from "./infra"

import { detectMenuTargetPath } from "./convert"

export function mockLoadMenuLocationDetecter(currentURL: URL, version: string): LoadMenuDetecter {
    return () => detectMenuTargetPath(currentURL, version)
}
