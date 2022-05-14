import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { RegisterAuthUserAccountConfig } from "../action"

export function newRegisterAuthUserAccountConfig(): RegisterAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: AUTH_CONFIG.resetToInitialTimeout,
    }
}
