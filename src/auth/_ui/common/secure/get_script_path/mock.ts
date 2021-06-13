import { GetScriptPathDetecter } from "./method"

import { detectPathname } from "./converter"

export function mockGetScriptPathDetecter(currentURL: URL): GetScriptPathDetecter {
    return () => detectPathname(currentURL)
}
