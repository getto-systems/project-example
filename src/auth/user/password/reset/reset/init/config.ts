import { AUTH_CONFIG } from "../../../../../x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../../../sign/get_script_path/init/config"
import { newStartContinuousRenewConfig } from "../../../../../ticket/start_continuous_renew/init/config"

import { ResetPasswordConfig } from "../action"

export function newResetPasswordConfig(): ResetPasswordConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
