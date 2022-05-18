import { COMMON_CONFIG } from "../../../../../../../common/x_outside_feature/config"

import { ChangeResetTokenDestinationConfig } from "../action"

export function newChangeResetTokenDestinationConfig(): ChangeResetTokenDestinationConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: COMMON_CONFIG.resetToInitialTimeout,
    }
}
