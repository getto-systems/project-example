import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { SearchAuthUserAccountConfig } from "../action"

export function newSearchAuthUserAccountConfig(): SearchAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}
