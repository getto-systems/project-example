import { GetScriptPathDetecter } from "./method"

import { detectPathname } from "./convert"

export function mockGetScriptPathDetecter(currentURL: URL): GetScriptPathDetecter {
    return () => detectPathname(currentURL)
}
