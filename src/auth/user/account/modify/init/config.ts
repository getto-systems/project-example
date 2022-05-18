import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { ModifyAuthUserAccountConfig } from "../action"

export function newModifyAuthUserAccountConfig(): ModifyAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: COMMON_CONFIG.resetToInitialTimeout,
    }
}
