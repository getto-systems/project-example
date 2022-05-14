import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { ChangePasswordConfig, OverwritePasswordConfig } from "../action"

export function newChangePasswordConfig(): ChangePasswordConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: AUTH_CONFIG.resetToInitialTimeout,
    }
}

export function newOverwritePasswordConfig(): OverwritePasswordConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
        resetToInitialTimeout: AUTH_CONFIG.resetToInitialTimeout,
    }
}
