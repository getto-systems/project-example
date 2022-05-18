import { expireDay } from "../../z_lib/ui/config/infra"

export const CORE_CONFIG = {
    manualSetupSeasonExpire: expireDay(90),
} as const
