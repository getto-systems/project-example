import { AUTH_CONFIG } from "../../../../x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../../sign/get_script_path/init"
import { newStartContinuousRenewConfig } from "../../../../ticket/start_continuous_renew/config"

import { AuthenticatePasswordConfig } from "../action"

export function newAuthenticatePasswordConfig(): AuthenticatePasswordConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
