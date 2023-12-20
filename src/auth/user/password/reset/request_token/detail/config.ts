import { COMMON_CONFIG } from "../../../../../../common/x_outside_feature/config"

import { RequestResetTokenConfig } from "../action"

export function newRequestResetTokenConfig(): RequestResetTokenConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}
