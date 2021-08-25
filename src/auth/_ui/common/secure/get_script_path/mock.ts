import { GetScriptPathDetecter } from "./method"

import { detectPathname } from "./convert"

import { SecureServerURL } from "./infra"

export function mockGetScriptPathDetecter(currentURL: URL): GetScriptPathDetecter {
    return () => detectPathname(currentURL)
}

export function mockSecureServerURL(url: string): SecureServerURL {
    return url as SecureServerURL
}
