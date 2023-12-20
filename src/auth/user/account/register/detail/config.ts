import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { RegisterAuthUserAccountConfig } from "../action"

export function newRegisterAuthUserAccountConfig(): RegisterAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: COMMON_CONFIG.resetToInitialTimeout,
    }
}
