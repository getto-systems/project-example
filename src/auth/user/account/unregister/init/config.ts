import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { UnregisterAuthUserAccountConfig } from "../action"

export function newUnregisterAuthUserAccountConfig(): UnregisterAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
