import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { UnregisterAuthUserAccountConfig } from "../action"

export function newUnregisterAuthUserAccountConfig(): UnregisterAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}
