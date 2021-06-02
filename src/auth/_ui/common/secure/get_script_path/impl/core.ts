import { pathnameLocationConverter, scriptPathConverter } from "./converter"

import { GetScriptPathLocationMethod, GetScriptPathPod } from "../method"

import { GetScriptPathInfra } from "../infra"

export const detectPathname: GetScriptPathLocationMethod = (currentURL: URL) =>
    pathnameLocationConverter(currentURL)

interface GetSecureScriptPath {
    (infra: GetScriptPathInfra): GetScriptPathPod
}
export const getScriptPath: GetSecureScriptPath = (infra) => (detecter) => () => {
    const { config } = infra

    const pathname = detecter()
    if (!pathname.valid) {
        return { valid: false }
    }

    return scriptPathConverter(config.secureServerURL, pathname.value)
}
