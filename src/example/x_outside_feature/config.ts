import { expireDay } from "../../z_lib/ui/config/infra"

export const EXAMPLE_CONFIG = {
    focusSeasonExpire: expireDay(90),
} as const
