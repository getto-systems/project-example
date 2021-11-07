import { delaySecond, expireMinute, intervalMinute } from "../../z_lib/ui/config/infra"

export const auth_config = {
    instantLoadExpire: expireMinute(3),
    authnExpire: expireMinute(1),
    continuousRenewInterval: intervalMinute(2),
    takeLongtimeThreshold: delaySecond(1),
} as const
