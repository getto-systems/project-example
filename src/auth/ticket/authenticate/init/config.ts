import { AUTH_CONFIG } from "../../../x_outside_feature/config"
import { COMMON_CONFIG } from "../../../../common/x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../sign/get_script_path/init/config"
import { StartContinuousRenewConfig } from "../method"

import { AuthenticateWithTokenConfig } from "../action"

export function newCheckAuthTicketConfig(): AuthenticateWithTokenConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        instantLoadExpire: AUTH_CONFIG.instantLoadExpire,
        takeLongtimeThreshold: COMMON_CONFIG.takeLongtimeThreshold,
    }
}

export function newStartContinuousRenewConfig(): StartContinuousRenewConfig {
    return {
        ticketExpire: AUTH_CONFIG.ticketExpire,
        continuousRenewInterval: AUTH_CONFIG.continuousRenewInterval,
    }
}
