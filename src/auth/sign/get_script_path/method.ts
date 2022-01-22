import { GetScriptPathConfig, GetScriptPathShell } from "./infra"

import { toScriptPath } from "./convert"

import { ConvertScriptPathResult } from "./data"

export type GetScriptPathMaterial = Readonly<{
    shell: GetScriptPathShell
    config: GetScriptPathConfig
}>

export function getScriptPath({ shell, config }: GetScriptPathMaterial): ConvertScriptPathResult {
    const pathname = shell.detectLocationPathname()
    if (!pathname.valid) {
        return { valid: false }
    }

    return { valid: true, value: toScriptPath(pathname.value, config.secureServerURL) }
}
