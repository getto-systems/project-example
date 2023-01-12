import { COMMON_CONFIG } from "../../../../../common/x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../../sign/get_script_path/init/config"
import { newStartContinuousRenewConfig } from "../../../../ticket/authenticate/init/config"

import { AuthenticatePasswordConfig } from "../action"

export function newAuthenticatePasswordConfig(): AuthenticatePasswordConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}
