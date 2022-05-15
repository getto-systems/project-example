import { AUTH_CONFIG } from "../../../../../x_outside_feature/config"

import { RequestResetTokenConfig } from "../action"

export function newRequestResetTokenConfig(): RequestResetTokenConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
