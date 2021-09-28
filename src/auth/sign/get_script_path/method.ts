import { GetScriptPathInfra } from "./infra"

import { toScriptPath } from "./convert"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { ConvertScriptPathResult, LocationPathname } from "./data"

export interface GetScriptPathDetecter {
    (): ConvertLocationResult<LocationPathname>
}

export interface GetScriptPathMethod {
    (pathname: ConvertLocationResult<LocationPathname>): ConvertScriptPathResult
}

interface GetSecureScriptPath {
    (infra: GetScriptPathInfra): GetScriptPathMethod
}
export const getScriptPath: GetSecureScriptPath = (infra) => (pathname) => {
    const { config } = infra

    if (!pathname.valid) {
        return { valid: false }
    }

    return { valid: true, value: toScriptPath(pathname.value, config.secureServerURL) }
}
