import { expireDay } from "../../z_lib/ui/config/infra"

export const EXAMPLE_CONFIG = {
    manualSetupSeasonExpire: expireDay(90),
} as const
