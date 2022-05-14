import { expireDay, waitSecond } from "../../z_lib/ui/config/infra"

export const CORE_CONFIG = {
    manualSetupSeasonExpire: expireDay(90),
    resetToInitialTimeout: waitSecond(1),
} as const
