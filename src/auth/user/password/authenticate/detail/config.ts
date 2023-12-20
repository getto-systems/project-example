import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../../sign/get_script_path/detail/config"
import { newStartContinuousRenewConfig } from "../../../../ticket/authenticate/detail/config"

import { AuthenticatePasswordConfig } from "../action"

export function newAuthenticatePasswordConfig(): AuthenticatePasswordConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}
