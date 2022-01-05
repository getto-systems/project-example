import { AUTH_CONFIG } from "../../../x_outside_feature/config"

import { newGetScriptPathConfig } from "../../../sign/get_script_path/init/config"
import { newStartContinuousRenewConfig } from "../../start_continuous_renew/init/config"

import { CheckAuthTicketConfig } from "../action"

export function newCheckAuthTicketConfig(): CheckAuthTicketConfig {
    return {
        ...newStartContinuousRenewConfig(),
        ...newGetScriptPathConfig(),
        instantLoadExpire: AUTH_CONFIG.instantLoadExpire,
        takeLongtimeThreshold: AUTH_CONFIG.takeLongtimeThreshold,
    }
}
