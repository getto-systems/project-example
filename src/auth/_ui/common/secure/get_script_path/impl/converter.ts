import { LocationConverter } from "../../../../../../../ui/vendor/getto-application/location/infra"

import { ConvertScriptPathResult, LocationPathname, ScriptPath } from "../data"

type PathnameConverter = LocationConverter<LocationPathname, URL>
export const pathnameLocationConverter: PathnameConverter = (currentURL) => {
    const pathname = currentURL.pathname
    if (!pathname.endsWith(".html")) {
        return { valid: false }
    }
    return { valid: true, value: markLocationPathname(pathname) }
}

export function scriptPathConverter(
    secureServerURL: string,
    pathname: LocationPathname,
): ConvertScriptPathResult {
    // アクセス中の html と同じパスで secure host に js がホストされている
    const scriptPath = pathname.replace(/\.html$/, ".js")
    return {
        valid: true,
        value: markSecureScriptPath(`${secureServerURL}${scriptPath}`),
    }
}

function markLocationPathname(pathname: string): LocationPathname {
    return pathname as LocationPathname
}
function markSecureScriptPath(path: string): ScriptPath {
    return path as ScriptPath
}
