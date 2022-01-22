import { env } from "../../../../y_environment/ui/env"

import { AVAIL_CONFIG } from "../../../x_outside_feature/config"

import { FindNextVersionConfig } from "../action"

export function newFindNextVersionConfig(): FindNextVersionConfig {
    return {
        version: env.version,
        versionSuffix: env.versionSuffix,
        takeLongtimeThreshold: AVAIL_CONFIG.takeLongtimeThreshold,
    }
}
