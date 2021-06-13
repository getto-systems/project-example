import { ConvertLocationResult } from "../../../../../z_details/_ui/location/data"

import { LocationPathname, ScriptPath } from "./data"

export function detectPathname(currentURL: URL): ConvertLocationResult<LocationPathname> {
    const pathname = currentURL.pathname
    if (!pathname.endsWith(".html")) {
        return { valid: false }
    }
    return { valid: true, value: pathname as LocationPathname }
}

export function toScriptPath(
    pathname: LocationPathname,
    config: Readonly<{ secureServerURL: string }>,
): ScriptPath {
    // アクセス中の html と同じパスで secure host に js がホストされている
    // pathname は必ず html でおわる(see: detectPathname)
    const scriptPath = pathname.replace(/\.html$/, ".js")
    return `${config.secureServerURL}${scriptPath}` as ScriptPath
}
