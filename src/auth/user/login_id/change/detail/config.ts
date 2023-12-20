import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { OverwriteLoginIdConfig } from "../action"

export function newOverwritePasswordConfig(): OverwriteLoginIdConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: COMMON_CONFIG.resetToInitialTimeout,
    }
}
