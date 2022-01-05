import { AUTH_CONFIG } from "../../x_outside_feature/config"

import { StartContinuousRenewConfig } from "./infra"

export function newStartContinuousRenewConfig(): StartContinuousRenewConfig {
    return {
        ticketExpire: AUTH_CONFIG.ticketExpire,
        continuousRenewInterval: AUTH_CONFIG.continuousRenewInterval,
    }
}
