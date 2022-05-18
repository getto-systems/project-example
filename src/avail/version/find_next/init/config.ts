import { env } from "../../../../y_environment/ui/env"

import { COMMON_CONFIG } from "../../../../common/x_outside_feature/config"

import { FindNextVersionConfig } from "../action"

export function newFindNextVersionConfig(): FindNextVersionConfig {
    return {
        version: env.version,
        versionSuffix: env.versionSuffix,
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}
