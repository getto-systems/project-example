import { COMMON_CONFIG } from "../../../x_outside_feature/config"

import { SetupSeasonConfig } from "../action"

export function newSetupSeasonConfig(): SetupSeasonConfig {
    return {
        manualSetupSeasonExpire: COMMON_CONFIG.manualSetupSeasonExpire,
        resetToInitialTimeout: COMMON_CONFIG.resetToInitialTimeout,
    }
}
