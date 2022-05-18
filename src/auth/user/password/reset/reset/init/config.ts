import { COMMON_CONFIG } from "../../../../../../common/x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../../../sign/get_script_path/init/config"
import { newStartContinuousRenewConfig } from "../../../../../ticket/check/init/config"

import { ResetPasswordConfig } from "../action"

export function newResetPasswordConfig(): ResetPasswordConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}
