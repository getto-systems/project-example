import { AUTH_CONFIG } from "../../../x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../sign/get_script_path/init/config"
import { StartContinuousRenewConfig } from "../../check/method"

import { CheckAuthTicketConfig } from "../action"

export function newCheckAuthTicketConfig(): CheckAuthTicketConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        instantLoadExpire: AUTH_CONFIG.instantLoadExpire,
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}

export function newStartContinuousRenewConfig(): StartContinuousRenewConfig {
    return {
        ticketExpire: AUTH_CONFIG.ticketExpire,
        continuousRenewInterval: AUTH_CONFIG.continuousRenewInterval,
    }
}
