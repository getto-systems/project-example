import { detectPathname } from "./convert"

import { SecureServerURL, GetScriptPathShell } from "./infra"

export function mockGetScriptPathShell(currentURL: URL): GetScriptPathShell {
    return {
        detectLocationPathname: () => detectPathname(currentURL),
    }
}

export function mockSecureServerURL(url: string): SecureServerURL {
    return url as SecureServerURL
}
