import { EXAMPLE_CONFIG } from "../../../x_outside_feature/config"

import { SetupSeasonConfig } from "../action"

export function newSetupSeasonConfig(): SetupSeasonConfig {
    return {
        manualSetupSeasonExpire: EXAMPLE_CONFIG.manualSetupSeasonExpire,
    }
}
