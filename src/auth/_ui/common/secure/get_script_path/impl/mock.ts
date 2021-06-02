import { mockLocationDetecter } from "../../../../../../../ui/vendor/getto-application/location/mock"

import { detectPathname } from "./core"

import { GetScriptPathLocationDetecter } from "../method"

export function mockGetScriptPathLocationDetecter(currentURL: URL): GetScriptPathLocationDetecter {
    return mockLocationDetecter(currentURL, detectPathname)
}
