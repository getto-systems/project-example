import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { ChangePasswordConfig } from "../action"

export function newChangePasswordConfig(): ChangePasswordConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
