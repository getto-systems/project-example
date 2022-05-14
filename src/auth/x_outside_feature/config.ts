import { expireMinute, intervalMinute, waitSecond } from "../../z_lib/ui/config/infra"

export const AUTH_CONFIG = {
    instantLoadExpire: expireMinute(3),
    ticketExpire: expireMinute(1),
    continuousRenewInterval: intervalMinute(2),
    takeLongtimeThreshold: waitSecond(1),
    resetToInitialTimeout: waitSecond(1),
} as const
