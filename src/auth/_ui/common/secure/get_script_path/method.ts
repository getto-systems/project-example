import { GetScriptPathInfra } from "./infra"

import { toScriptPath } from "./converter"

import { ConvertLocationResult } from "../../../../../z_details/_ui/location/data"
import { ConvertScriptPathResult, LocationPathname } from "./data"

export interface GetScriptPathPod {
    (detecter: GetScriptPathDetecter): GetScriptPathMethod
}

export type GetScriptPathDetecter = Detect<LocationPathname>

export interface GetScriptPathMethod {
    (): ConvertScriptPathResult
}

interface GetSecureScriptPath {
    (infra: GetScriptPathInfra): GetScriptPathPod
}
export const getScriptPath: GetSecureScriptPath = (infra) => (detecter) => () => {
    const { config } = infra

    const pathname = detecter()
    if (!pathname.valid) {
        return { valid: false }
    }

    return { valid: true, value: toScriptPath(pathname.value, config) }
}

interface Detect<T> {
    (): ConvertLocationResult<T>
}
