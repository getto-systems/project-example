import { GetScriptPathConfig, GetScriptPathShell } from "./infra"

import { toScriptPath } from "./convert"

import { ConvertScriptPathResult } from "./data"

export function getScriptPath(
    config: GetScriptPathConfig,
    shell: GetScriptPathShell,
): ConvertScriptPathResult {
    const pathname = shell.detectLocationPathname()
    if (!pathname.valid) {
        return { valid: false }
    }

    return { valid: true, value: toScriptPath(pathname.value, config.secureServerURL) }
}
