import { detectSignViewType } from "./convert"

import { SignViewDetecter } from "./data"

export function mockSignViewLocationDetecter(currentURL: URL): SignViewDetecter {
    return () => detectSignViewType(currentURL)
}
