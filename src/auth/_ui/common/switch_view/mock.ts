import { detectSignViewType } from "./core"

import { SignViewDetecter } from "./data"

export function mockSignViewLocationDetecter(currentURL: URL): SignViewDetecter {
    return () => detectSignViewType(currentURL)
}
