import { expireMinute, intervalMinute } from "../../common/util/config/infra"

export const AUTH_CONFIG = {
    instantLoadExpire: expireMinute(3),
    ticketExpire: expireMinute(1),
    continuousRenewInterval: intervalMinute(2),
} as const
