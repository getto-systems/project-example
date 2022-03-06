import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { ChangePasswordConfig, OverridePasswordConfig } from "../action"

export function newChangePasswordConfig(): ChangePasswordConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}

export function newOverridePasswordConfig(): OverridePasswordConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
