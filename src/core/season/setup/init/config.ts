import { CORE_CONFIG } from "../../../x_outside_feature/config"

import { SetupSeasonConfig } from "../action"

export function newSetupSeasonConfig(): SetupSeasonConfig {
    return {
        manualSetupSeasonExpire: CORE_CONFIG.manualSetupSeasonExpire,
    }
}
