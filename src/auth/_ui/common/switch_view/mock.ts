import { mockDetecter } from "../../../../../ui/vendor/getto-application/location/mock"

import { detectSignViewType } from "./core"

import { SignViewLocationDetecter } from "./data"

export function mockSignViewLocationDetecter(currentURL: URL): SignViewLocationDetecter {
    return mockDetecter(currentURL, detectSignViewType)
}
