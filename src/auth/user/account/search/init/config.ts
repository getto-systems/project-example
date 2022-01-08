import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { SearchAuthUserAccountConfig } from "../action"

export function newSearchAuthUserAccountConfig(): SearchAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
