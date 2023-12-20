import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { ChangePasswordConfig, OverwritePasswordConfig } from "../action"

export function newChangePasswordConfig(): ChangePasswordConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: COMMON_CONFIG.resetToInitialTimeout,
    }
}

export function newOverwritePasswordConfig(): OverwritePasswordConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: COMMON_CONFIG.resetToInitialTimeout,
    }
}
