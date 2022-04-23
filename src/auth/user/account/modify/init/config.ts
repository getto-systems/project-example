import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { ModifyAuthUserAccountConfig } from "../action"

export function newModifyAuthUserAccountConfig(): ModifyAuthUserAccountConfig {
    return {
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
