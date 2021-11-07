import { detectPathname } from "./convert"

import { SecureServerURL, GetScriptPathDetecter } from "./infra"

export function mockGetScriptPathDetecter(currentURL: URL): GetScriptPathDetecter {
    return () => detectPathname(currentURL)
}

export function mockSecureServerURL(url: string): SecureServerURL {
    return url as SecureServerURL
}
