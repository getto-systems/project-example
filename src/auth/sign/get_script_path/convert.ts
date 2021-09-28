import { ConvertLocationResult } from "../../../z_lib/ui/location/data"

import { LocationPathname, ScriptPath } from "./data"
import { SecureServerURL } from "./infra"

export function detectPathname(currentURL: URL): ConvertLocationResult<LocationPathname> {
    const pathname = currentURL.pathname
    if (!pathname.endsWith(".html")) {
        return { valid: false }
    }
    return { valid: true, value: pathname as LocationPathname }
}

export function toScriptPath(
    pathname: LocationPathname,
    secureServerURL: SecureServerURL,
): ScriptPath {
    // アクセス中の html と同じパスで secure host に js がホストされている
    // pathname は必ず html でおわる(see: detectPathname)
    const scriptPath = pathname.replace(/\.html$/, ".js")
    return `${secureServerURL}${scriptPath}` as ScriptPath
}
